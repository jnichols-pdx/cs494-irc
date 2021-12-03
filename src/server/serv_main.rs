#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::*;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, sleep, Duration};

use std::collections::HashMap;
use std::io::ErrorKind;
use std::io::{stderr, Write};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering, AtomicU16};
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() -> Result<'static, ()> {
    println!("Hello, world! [server]");
    let running = Arc::new(AtomicBool::new(true));
    let r1 = running.clone();
    let r2 = running.clone();
    ctrlc::set_handler(move || {
        r1.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let listener = TcpListener::bind("0.0.0.0:17734").await?;

    let master_rooms = Arc::new(RwLock::new(HashMap::new())); //<String, RoomHandle>
    let master_users = Arc::new(RwLock::new(HashMap::new())); //<String, ClientHandle>
    let master_transfers = Arc::new(RwLock::new(HashMap::new())); //<Int, TransferNotes>

    let mrc = master_rooms.clone();
    let muc = master_users.clone();
    let mtc = master_transfers.clone();

    let transfer_count = Arc::new(AtomicU16::new(0));
    let tc1 = transfer_count.clone();

    //let offline_message;
    //blocks until one of the tasks listed returns
    tokio::select! {
        //out = listener_task => {},//offline_message = format!("Response: {:?}",out?);},
        //_ = stop_task => {},//offline_message = "User asked to quit.".into();},
        out = new_connections(listener, mrc, muc, tc1, mtc) => {},
        _ = shutdown_monitor(r2) => {},
    }

    let final_users = master_users.write().unwrap(); //grab exclusive access to user to say our goodbyes.

    //println!("about to exit");
    for (_, client_handle) in final_users.iter() {
        let outgoing = ServerDepartsPacket::new(&"Server going down for maintenance.".to_string())
            .expect("Server closing anyway.");
        client_handle
            .send_channel_sink
            .send(outgoing.into())
            .await?;
    }
    sleep(Duration::from_millis(1000)).await;
    Ok(())
}

async fn shutdown_monitor(running: Arc<AtomicBool>) {
    let mut wait_period = time::interval(Duration::from_millis(100));
    loop {
        wait_period.tick().await;
        if !running.load(Ordering::SeqCst) {
            println!("\nCaught SIGINT, shutting down.");
            //ctrl-c was pressed, break to signal we should shutdown.
            //println!("Detected ctl-c");
            break;
        }
    }
}

async fn new_connections<'a>(
    listener: TcpListener,
    master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
    master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
    transfer_count: Arc<AtomicU16>,
    master_transfers: Arc<RwLock<HashMap<u16, TransferNotes>>>,
) -> Result<'a, ()> {
    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut peeker = [0; 5];
        let mut bytes_peeked;
        bytes_peeked = socket.peek(&mut peeker).await?;
        if bytes_peeked == 5 {
            let msg_len = u32_from_slice(&peeker[1..5]) as usize;
            let mut buffer = vec![0; msg_len + 5];
            let bytes_read = socket.read(&mut buffer).await?;
            if bytes_read == msg_len + 5 {
                let kind_raw = IrcKind::from(buffer[0]);
                match kind_raw {
                    IrcKind::IRC_KIND_NEW_CLIENT => {
                        let new_client = NewClientPacket::from_bytes(&buffer)?;
                        let master_users_copy = master_users.clone();
                        let master_rooms_copy = master_rooms.clone();
                        let master_transfers_copy = master_transfers.clone();
                        let mut should_reject;
                        {
                            let mut master_users_ro = master_users.read().unwrap();
                            should_reject = master_users_ro.contains_key(&new_client.chat_name);
                        }
                        if should_reject {
                            println!("Rejecting duplicate user: {}", new_client.chat_name);
                            socket
                                .write(
                                    &ErrorPacket::new(IrcErrCode::IRC_ERR_NAME_IN_USE)?.as_bytes(),
                                )
                                .await?;
                            socket.shutdown().await?;
                        } else {
                            println!("New client connected: '{}'", new_client.chat_name);
                            //Spin up a new user
                            socket.set_nodelay(true).expect("Unable to set delay false");
                            let (channel_sink, mut channel_source) =
                                mpsc::channel::<SyncSendPack>(64);
                            //let handle_sink = channel_sink.clone();

                            let client_name = new_client.chat_name.clone();
                            let new_client_handle1 = ClientHandle {
                                send_channel_sink: channel_sink,
                                name: new_client.chat_name,
                            };

                            let new_client_handle2 = new_client_handle1.clone();

                            {
                                let mut master_users_rw = master_users.write().unwrap();
                                master_users_rw.insert(client_name, new_client_handle1);
                            }
                            tokio::spawn(client_lifecycle(
                                socket,
                                master_rooms_copy,
                                master_users_copy,
                                new_client_handle2,
                                channel_source,
                                transfer_count.clone(),
                                master_transfers_copy,
                            ));
                        }
                    }
                    _ => {
                        let _ =  writeln!(stderr(),"Error: Expected New Client Packet for new connection, received:\n{:?}\n",&buffer[0..bytes_read]);
                        let error_notice = ErrorPacket::new(IrcErrCode::IRC_ERR_UNKNOWN)
                            .expect("Error packets should be infallible on creation");
                        socket.write(&error_notice.as_bytes()).await?;
                        socket.shutdown().await?;
                    }
                }
            }
        }
    }
}

async fn client_lifecycle<'a>(
    mut socket: TcpStream,
    master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
    master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
    mut our_handle: ClientHandle,
    mut channel_source: mpsc::Receiver<SyncSendPack>,
    transfer_count: Arc<AtomicU16>,
    master_transfers: Arc<RwLock<HashMap<u16, TransferNotes>>>,
) -> Result<'a, ()> {
    //Split the TcpStream into reader and writer, pass each to their own asynchronous task
    let (tcp_in, tcp_out) = socket.into_split();
    let client_name = our_handle.name;
    let channel_sink = our_handle.send_channel_sink;
    let sink1 = channel_sink.clone();
    let sink2 = channel_sink.clone();
    let sink3 = channel_sink.clone();
    let found_pulse = Arc::new(AtomicBool::new(true));
    let fp = found_pulse.clone();
    let (responder_sink, mut responder_source) = mpsc::channel::<SyncSendPack>(32);
    let mrc = master_rooms.clone();
    let muc = master_users.clone();
    let mtc = master_transfers.clone();

    let offline_message: String;
    tokio::select! {
        out = reader(tcp_in, responder_sink, fp, sink1) => {
            match out {
                Ok(msg) => offline_message = msg,
                Err(e) => offline_message = format!("{}",e,),
            };
        },
        out = responder(client_name.clone(), responder_source, sink3, mrc, muc, transfer_count, mtc) => {
            match out {
                Ok(msg) => offline_message = msg,
                Err(e) => offline_message = format!("{}",e,),
            };
        },
        _ = writer(tcp_out, channel_source) => {offline_message = "Downstream connection ended.".into();},
        _ = pulse(sink2) => {offline_message = "Internal Error (server keepalive failed).".into();},
        _ = pulse_monitor(found_pulse) => {offline_message = "No heartbeat responded in 30 seconds.".into();},
    }

    println!("Client '{}' ejected: {}", &client_name, &offline_message);
    {
        let mut master_users_rw = master_users.write().unwrap();
        master_users_rw.remove(&client_name);
    }

    //Don't have access to the cached_rooms tha the Responder uses to track which rooms this user
    //is in. Getting access would require wrapping it in an Arc<Mutex<>> for thread safety, and
    //we already have enough of that going on.
    //Instead send a notice to ALL room tasks that this user should be removed.
    let mut rooms_to_notify: Vec<mpsc::Sender<String>> = Vec::new();
    {
        let master_rooms_ro = master_rooms.read().unwrap();
        for (_, handle) in master_rooms_ro.iter() {
            rooms_to_notify.push(handle.leave_channel_sink.clone());
        }
    }

    for sink in rooms_to_notify {
        sink.send(client_name.clone()).await?;
    }
    Ok(())
}

async fn pulse<'a>(tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {
    let mut wait_period = time::interval(Duration::from_millis(5000));
    loop {
        wait_period.tick().await;
        let heartbeat =
            HeartbeatPacket::new().expect("Heartbeat packets should be infallible on creation");
        tx_packet_out.send(heartbeat.into()).await?;
    }
}

async fn pulse_monitor<'a>(found_pulse: Arc<AtomicBool>) -> Result<'a, ()> {
    let mut seconds_since_heartbeat = 0 as u8;
    let mut wait_period = time::interval(Duration::from_millis(1000));
    loop {
        wait_period.tick().await;
        if found_pulse.load(Ordering::SeqCst) {
            seconds_since_heartbeat = 0;
            found_pulse.store(false, Ordering::SeqCst);
        } else {
            if seconds_since_heartbeat >= 30 {
                break;
            }
            seconds_since_heartbeat += 1;
        }
    }

    Ok(())
}

async fn writer<'a>(
    mut con: tokio::net::tcp::OwnedWriteHalf,
    mut rx_packets_to_send: mpsc::Receiver<SyncSendPack>,
) -> Result<'a, ()> {
    let mut bytes_to_go;
    let mut should_stop = false;
    while let Some(sync_send_packet) = rx_packets_to_send.recv().await {
        //println!("send me packets!");
        match sync_send_packet.contained_kind {
            IrcKind::IRC_KIND_ERR => {
                bytes_to_go = sync_send_packet.errp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_NEW_CLIENT => {
                bytes_to_go = sync_send_packet.ncp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_HEARTBEAT => {
                bytes_to_go = sync_send_packet.hbp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_ENTER_ROOM => {
                bytes_to_go = sync_send_packet.erp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_LEAVE_ROOM => {
                bytes_to_go = sync_send_packet.lrp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_LIST_ROOMS => {
                bytes_to_go = sync_send_packet.lip.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_ROOM_LISTING => {
                bytes_to_go = sync_send_packet.rlp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_USER_LISTING => {
                bytes_to_go = sync_send_packet.ulp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_QUERY_USER => {
                bytes_to_go = sync_send_packet.qup.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_SEND_MESSAGE => {
                bytes_to_go = sync_send_packet.smp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_BROADCAST_MESSAGE => {
                bytes_to_go = sync_send_packet.bmp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_POST_MESSAGE => {
                bytes_to_go = sync_send_packet.pmp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                bytes_to_go = sync_send_packet.dmp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_OFFER_FILE => {
                bytes_to_go = sync_send_packet.ofp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_ACCEPT_FILE => {
                bytes_to_go = sync_send_packet.afp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_REJECT_FILE => {
                bytes_to_go = sync_send_packet.rfp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_FILE_TRANSFER => {
                bytes_to_go = sync_send_packet.ftp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_CLIENT_DEPARTS => {
                bytes_to_go = sync_send_packet.cdp.unwrap().as_bytes();
            }
            IrcKind::IRC_KIND_SERVER_DEPARTS => {
                bytes_to_go = sync_send_packet.sdp.unwrap().as_bytes();
                should_stop = true;
            }
            _ => {
                println!("Can't send Unknown type packet!");
                continue;
            }
        }
        con.write(&bytes_to_go).await?;
        con.flush().await?;
        if should_stop {
            break;
        }
    }
    con.shutdown().await?;
    Ok(())
}

async fn reader<'a>(
    mut con: tokio::net::tcp::OwnedReadHalf,
    tx_to_responder: mpsc::Sender<SyncSendPack>,
    found_pulse: Arc<AtomicBool>,
    tx_packet_out: mpsc::Sender<irclib::SyncSendPack>,
) -> Result<'a, String> {
    let mut peeker = [0; 5];
    let mut bytes_peeked;
    let mut ret_string = "Unexpected connection closure.".to_string();
    loop {
        bytes_peeked = con.peek(&mut peeker).await?;
        if bytes_peeked == 5 {
            let msg_len = u32_from_slice(&peeker[1..5]) as usize;
            let mut buffer = vec![0; msg_len + 5];
            let bytes_read = con.read(&mut buffer).await?;
            if bytes_read == msg_len + 5 {
                let kind_raw = IrcKind::from(buffer[0]);
                match kind_raw {
                    IrcKind::IRC_KIND_ERR => {
                        let my_error = ErrorPacket::from_bytes(&buffer[0..6])?;
                        match my_error.error_code {
                            IrcErrCode::IRC_ERR_UNKNOWN => {
                                ret_string =
                                    "Bogus! Client's confused (we received Error: Unknown)".into();
                            }
                            IrcErrCode::IRC_ERR_ILLEGAL_KIND => {
                                ret_string = "Bogus! Illegal Kind!".into();
                            }
                            IrcErrCode::IRC_ERR_ILLEGAL_LENGTH => {
                                ret_string = "Bogus! Illegal Length!".into();
                            }
                            IrcErrCode::IRC_ERR_NAME_IN_USE => {
                                ret_string = "Bogus! That name's taken!".into();
                            }
                            IrcErrCode::IRC_ERR_ILLEGAL_NAME => {
                                ret_string = "Bogus! Illegal Name!".into();
                            }
                            IrcErrCode::IRC_ERR_ILLEGAL_MESSAGE => {
                                ret_string = "Bogus! Illegal Message!".into();
                            }
                            IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER => {
                                ret_string = "Bogus! Illegal Transfer!".into();
                            }
                            IrcErrCode::IRC_ERR_TOO_MANY_USERS => {
                                ret_string = "Bogus! Slashdoted! (too many users)".into();
                            }
                            IrcErrCode::IRC_ERR_TOO_MANY_ROOMS => {
                                ret_string = "Bogus! Too Many Rooms!".into();
                            }
                            _ => (),
                        }
                        break;
                    }
                    IrcKind::IRC_KIND_NEW_CLIENT => { /*println!("Got New client packet...from an already connected client?");*/
                    }
                    IrcKind::IRC_KIND_HEARTBEAT => {
                        found_pulse.store(true, Ordering::SeqCst);
                    }
                    IrcKind::IRC_KIND_ENTER_ROOM => {
                        let enter_room = EnterRoomPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(enter_room.into()).await?;
                    }
                    IrcKind::IRC_KIND_LEAVE_ROOM => {
                        let leave_room = LeaveRoomPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(leave_room.into()).await?;
                    }
                    IrcKind::IRC_KIND_LIST_ROOMS => {
                        let list_rooms = ListRoomsPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(list_rooms.into()).await?;
                    }
                    IrcKind::IRC_KIND_ROOM_LISTING => { /*println!("Got room listing packet...?");*/
                    }
                    IrcKind::IRC_KIND_USER_LISTING => { /*println!("Got user listing packet...?");*/
                    }
                    IrcKind::IRC_KIND_QUERY_USER => {
                        let query_result = QueryUserPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(query_result.into()).await?;
                    }
                    IrcKind::IRC_KIND_SEND_MESSAGE => {
                        let send_message = SendMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(send_message.into()).await?;
                    }
                    IrcKind::IRC_KIND_BROADCAST_MESSAGE => {
                        let broadcast_message = BroadcastMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(broadcast_message.into()).await?;
                    }
                    IrcKind::IRC_KIND_POST_MESSAGE => { /*println!("Got user listing packet...?");*/
                    }
                    IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                        let new_direct = DirectMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_direct.into()).await?;
                    }
                    IrcKind::IRC_KIND_OFFER_FILE => {
                        let new_offer = OfferFilePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_offer.into()).await?;
                    }
                    IrcKind::IRC_KIND_ACCEPT_FILE => {
                        let new_accept = AcceptFilePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_accept.into()).await?;
                    }
                    IrcKind::IRC_KIND_REJECT_FILE => {
                        let new_reject = RejectFilePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_reject.into()).await?;
                    }
                    IrcKind::IRC_KIND_FILE_TRANSFER => {
                        let new_file_transfer = FileTransferPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_file_transfer.into()).await?;
                    }
                    IrcKind::IRC_KIND_CLIENT_DEPARTS => {
                        let client_leaving = ClientDepartsPacket::from_bytes(&buffer[..])?;
                        ret_string = format!(
                            "Client disconnected with this message: \"{}\"",
                            client_leaving.get_message()
                        );
                        break;
                    }
                    IrcKind::IRC_KIND_SERVER_DEPARTS => { /*println!("Got server departs packet...?");*/
                    }
                    _ => {
                        let _ = writeln!(
                            stderr(),
                            "Error: Unknown packet recieved:\n{:?}\n",
                            &buffer[0..bytes_read]
                        );
                        let error_notice = ErrorPacket::new(IrcErrCode::IRC_ERR_UNKNOWN)
                            .expect("Error packets should be infallible on creation");
                        tx_packet_out.send(error_notice.into()).await?;
                        break;
                    }
                }
            }
        } else {
            if bytes_peeked == 0 {
                //println!("Read connection to server has closed.");
                ret_string = "Read connection to client has closed.".into();
                break;
            }
        }
    }
    Ok(ret_string.into())
}

async fn responder<'a>(
    client_name: String,
    mut packet_source: mpsc::Receiver<SyncSendPack>,
    channel_sink: mpsc::Sender<irclib::SyncSendPack>,
    master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
    master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
    transfer_count: Arc<AtomicU16>,
    master_transfers: Arc<RwLock<HashMap<u16, TransferNotes>>>,
) -> Result<'a, String> {
    let mut cached_rooms: HashMap<String, RoomHandle> = HashMap::new(); //<String, RoomHandle>
    let mut cached_users: HashMap<String, ClientHandle> = HashMap::new(); //<String, ClientHandle>

    let ret_string: String = "".to_string();
    while let Some(packet) = packet_source.recv().await {
        match packet.contained_kind {
            IrcKind::IRC_KIND_ENTER_ROOM => {
                let erp = packet.erp.unwrap();
                let old_room_handle: Option<RoomHandle>;
                {
                    let master_rooms_ro;
                    match master_rooms.read() {
                        Ok(ro) => master_rooms_ro = ro,
                        Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                    }
                    old_room_handle = match master_rooms_ro.get(&erp.room_name) {
                        Some(rh) => {
                            //room exists, cache it for later
                            cached_rooms.insert(erp.room_name.clone(), rh.clone());
                            Some(rh.clone())
                        }
                        None => {
                            // need to make the room before we may join it
                            None
                        }
                    };
                }

                let handle_to_this_client = ClientHandle {
                    name: client_name.clone(),
                    send_channel_sink: channel_sink.clone(),
                };

                match old_room_handle {
                    Some(orh) => {
                        orh.join_channel_sink.send(handle_to_this_client).await?;
                    }
                    None => {
                        let new_room_handle = make_room(
                            erp.room_name.clone(),
                            master_users.clone(),
                            master_rooms.clone(),
                        )
                        .await?;
                        cached_rooms.insert(erp.room_name.clone(), new_room_handle.clone());
                        new_room_handle
                            .join_channel_sink
                            .send(handle_to_this_client)
                            .await?;
                    }
                };
                //Room will send its user list to the client after we join, indicating join success
                //to the client.
            }
            IrcKind::IRC_KIND_LEAVE_ROOM => {
                let lrp = packet.lrp.unwrap();
                match cached_rooms.get(&lrp.room_name) {
                    Some(rh) => {
                        rh.leave_channel_sink.send(client_name.clone()).await?;
                    }
                    None => {}
                };
            }
            IrcKind::IRC_KIND_LIST_ROOMS => {
                let mut outgoing = RoomListingPacket::new()?;
                {
                    let master_rooms_ro;
                    match master_rooms.read() {
                        Ok(ro) => master_rooms_ro = ro,
                        Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                    }
                    for (key, _) in master_rooms_ro.iter() {
                        outgoing.push(key)?;
                    }
                }
                channel_sink.send(outgoing.into()).await?;
            }
            IrcKind::IRC_KIND_QUERY_USER => {
                let qup = packet.qup.unwrap();
                let mut theyre_online;
                {
                    let master_users_ro;
                    match master_users.read() {
                        Ok(ro) => master_users_ro = ro,
                        Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                    }
                    match master_users_ro.get(&qup.user_name) {
                        Some(h) => theyre_online = true,
                        None => theyre_online = false,
                    }
                }
                let mut reply = QueryUserPacket::new(&qup.user_name)?;
                if theyre_online {
                    reply.set_online();
                } else {
                    reply.set_offline();
                }
                channel_sink.send(reply.into()).await?;
            }
            IrcKind::IRC_KIND_SEND_MESSAGE => {
                let smp = packet.smp.unwrap();
                let room = smp.room.clone();
                let message = smp.get_message();
                match cached_rooms.get(&room) {
                    Some(rh) => {
                        let post_message = PostMessagePacket::new(&room, &client_name, &message)?;
                        rh.post_channel_sink.send(post_message.into()).await?;
                    }
                    None => {}
                };
            }
            IrcKind::IRC_KIND_BROADCAST_MESSAGE => {
                let bmp = packet.bmp.unwrap();
                let message = bmp.get_message();
                for (room, handle) in &cached_rooms {
                    let post_message = PostMessagePacket::new(&room, &client_name, &message)?;
                    handle.post_channel_sink.send(post_message.into()).await?;
                }
            }
            IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                let dmp = packet.dmp.unwrap();
                let outgoing =
                    DirectMessagePacket::new(&client_name.clone(), &dmp.message.clone())?;
                let mut need_lookup = false;
                match cached_users.get(&dmp.target) {
                    Some(user) => {
                        match user.send_channel_sink.send(outgoing.clone().into()).await {
                            Ok(_) => {}
                            // Recipient may have logged off and back on - invalidating the
                            // cached handle.
                            Err(_) => {
                                cached_users.remove(&dmp.target);
                                need_lookup = true;
                            }
                        };
                    }
                    None => {
                        need_lookup = true;
                    }
                };

                if need_lookup {
                    let their_new_handle: Option<ClientHandle>;
                    {
                        let master_users_ro;
                        match master_users.read() {
                            Ok(ro) => master_users_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        their_new_handle = match master_users_ro.get(&dmp.target) {
                            Some(h) => {
                                cached_users.insert(dmp.target.clone(), h.clone());
                                Some(h.clone())
                            }
                            None => None,
                        }
                    }
                    match their_new_handle {
                        Some(handle) => {
                            handle.send_channel_sink.send(outgoing.into()).await?;
                        }
                        None => {
                            let mut reply = QueryUserPacket::new(&dmp.target)?;
                            reply.set_offline();
                            channel_sink.send(reply.into()).await?;
                        }
                    };
                }
            }

            //TODO: refactor this shit to use a function for FINDING their current handle.
            //TODO: incorporate reocrding transfer state for use with ftp packets
            IrcKind::IRC_KIND_OFFER_FILE => {
                let mut ofp = packet.ofp.unwrap();
                let recipient = ofp.get_to();
                //Did the sender represent themselves faithfully?
                if ofp.get_from().ne(&client_name) {
                    let error_reply = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER).unwrap();
                    channel_sink.send(error_reply.into()).await?;
                    break; //Closes connection to the offending user.
                }
                let new_transfer_id = transfer_count.fetch_add(1, Ordering::SeqCst);
                let new_transfer_notes = TransferNotes {
                    file_size: ofp.get_size(),
                    bytes_seen: 0,
                    accepted: false,
                    to_user: ofp.get_to(),
                    from_user: ofp.get_from(),
                };

                {
                        let mut master_transfers_rw;
                        match master_transfers.write() {
                            Ok(rw) => master_transfers_rw = rw,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        master_transfers_rw.insert(new_transfer_id.clone(), new_transfer_notes);
                }

                ofp.set_id(new_transfer_id.clone());

                let mut need_lookup = false;
                let mut did_send = false;
                match cached_users.get(&recipient) {
                    Some(user) => {
                        match user.send_channel_sink.send(ofp.clone().into()).await {
                            Ok(_) => {did_send = true;}
                            // Recipient may have logged off and back on - invalidating the
                            // cached handle.
                            Err(_) => {
                                cached_users.remove(&recipient);
                                need_lookup = true;
                            }
                        };
                    }
                    None => {
                        need_lookup = true;
                    }
                };

                if need_lookup {
                    let their_new_handle: Option<ClientHandle>;
                    {
                        let master_users_ro;
                        match master_users.read() {
                            Ok(ro) => master_users_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        their_new_handle = match master_users_ro.get(&recipient) {
                            Some(h) => {
                                cached_users.insert(recipient.clone(), h.clone());
                                Some(h.clone())
                            }
                            None => None,
                        }
                    }
                    match their_new_handle {
                        Some(handle) => {
                            handle.send_channel_sink.send(ofp.into()).await?;
                            did_send = true;
                        }
                        None => {
                            let mut reply = QueryUserPacket::new(&recipient)?;
                            reply.set_offline();
                            channel_sink.send(reply.into()).await?;
                        }
                    };
                }
                if !did_send {
                    let mut master_transfers_rw;
                    match master_transfers.write() {
                        Ok(rw) => master_transfers_rw = rw,
                        Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                    }
                    master_transfers_rw.remove(&new_transfer_id);
                }
            }
            IrcKind::IRC_KIND_ACCEPT_FILE => {
                let mut afp = packet.afp.unwrap();
                //attemptForwardWithQueryReply(&ofp, ofp.get_to(), &mut channel_sink, &mut master_rooms, &mut master_users, &mut cached_users).await?;
                let recipient = afp.get_to();
                let sender = afp.get_from();
                let transfer_id = afp.get_transfer_id();
                //Did the recipient represent themselves faithfully?
                if recipient.ne(&client_name) {
                    {
                        let mut master_transfers_rw;
                        match master_transfers.write() {
                            Ok(rw) => master_transfers_rw = rw,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        master_transfers_rw.remove(&transfer_id);
                    }

                    let error_reply = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER).unwrap();
                    channel_sink.send(error_reply.into()).await?;
                   
                    //TODO: handle notifying the transfer originator that the recipient is no
                    //longer avaiable. REALLY need a generic 'try sending thing to this user'
                    //function >.<
                    
                    break; //Closes connection to the offending user.
                }

                let mut valid_transfer = true;
                {
                        let mut master_transfers_ro;
                        match master_transfers.read() {
                            Ok(ro) => master_transfers_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        match master_transfers_ro.get(&transfer_id) {
                            Some(transfer) => {
                                valid_transfer = valid_transfer && transfer.to_user.ne(&afp.get_to());
                                valid_transfer = valid_transfer && transfer.from_user.ne(&afp.get_from());
                                valid_transfer = valid_transfer && transfer.file_size.ne(&afp.get_size());
                            }
                            None => valid_transfer = false,
                        }

                }
                if !valid_transfer{
                    {
                        let mut master_transfers_rw;
                        match master_transfers.write() {
                            Ok(rw) => master_transfers_rw = rw,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        master_transfers_rw.remove(&transfer_id);
                    }
                    let error_reply = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER).unwrap();
                    channel_sink.send(error_reply.into()).await?;
                    break; //Closes connection to the offending user.
                }

                let mut need_lookup = false;
                let mut did_send = false;
                match cached_users.get(&recipient) {
                    Some(user) => {
                        match user.send_channel_sink.send(afp.clone().into()).await {
                            Ok(_) => {did_send = true;}
                            // Recipient may have logged off and back on - invalidating the
                            // cached handle.
                            Err(_) => {
                                cached_users.remove(&sender);
                                need_lookup = true;
                            }
                        };
                    }
                    None => {
                        need_lookup = true;
                    }
                };

                if need_lookup {
                    let their_new_handle: Option<ClientHandle>;
                    {
                        let master_users_ro;
                        match master_users.read() {
                            Ok(ro) => master_users_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        their_new_handle = match master_users_ro.get(&sender) {
                            Some(h) => {
                                cached_users.insert(recipient.clone(), h.clone());
                                Some(h.clone())
                            }
                            None => None,
                        }
                    }
                    match their_new_handle {
                        Some(handle) => {
                            handle.send_channel_sink.send(afp.into()).await?;
                            did_send = true;
                        }
                        None => {
                            let mut reply = QueryUserPacket::new(&sender)?;
                            reply.set_offline();
                            channel_sink.send(reply.into()).await?;
                        }
                    };
                }
                {
                    let mut master_transfers_rw;
                    match master_transfers.write() {
                        Ok(rw) => master_transfers_rw = rw,
                        Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                    }
                    if did_send {
                        match master_transfers_rw.get_mut(&transfer_id){
                            Some(mut transfer) => {
                                transfer.accepted = true;
                            }
                            None => {/*TODO - handle transfer cancelling in the middle of all this*/},
                        }
                    } else{
                        master_transfers_rw.remove(&transfer_id);
                    }
                }

            }
            IrcKind::IRC_KIND_REJECT_FILE => {
                let mut rfp = packet.rfp.unwrap();
                //attemptForwardWithQueryReply(&ofp, ofp.get_to(), &mut channel_sink, &mut master_rooms, &mut master_users, &mut cached_users).await?;
                let recipient = rfp.get_to();
                let sender = rfp.get_from();
                let transfer_id = rfp.get_transfer_id();
                //Did the recipient represent themselves faithfully?
                if recipient.ne(&client_name) {
                    {
                        let mut master_transfers_rw;
                        match master_transfers.write() {
                            Ok(rw) => master_transfers_rw = rw,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        master_transfers_rw.remove(&transfer_id);
                    }
                    //HERE -- PROBLEM - DOS available, send spurious packet with NOT your name, and
                    //someone else's transfer id. Will kill their transfer!! BUG BUG BUG

                    let error_reply = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER).unwrap();
                    channel_sink.send(error_reply.into()).await?;
                   
                    //TODO: handle notifying the transfer originator that the recipient is no
                    //longer avaiable. REALLY need a generic 'try sending thing to this user'
                    //function >.<
                    
                    break; //Closes connection to the offending user.
                }

                let mut valid_transfer = true;
                {
                        let mut master_transfers_ro;
                        match master_transfers.read() {
                            Ok(ro) => master_transfers_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        match master_transfers_ro.get(&transfer_id) {
                            Some(transfer) => {
                                valid_transfer = valid_transfer && transfer.to_user.ne(&rfp.get_to());
                                valid_transfer = valid_transfer && transfer.from_user.ne(&rfp.get_from());
                                valid_transfer = valid_transfer && transfer.file_size.ne(&rfp.get_size());
                            }
                            None => valid_transfer = false,
                        }

                }
                //remove it always
                    {
                        let mut master_transfers_rw;
                        match master_transfers.write() {
                            Ok(rw) => master_transfers_rw = rw,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        master_transfers_rw.remove(&transfer_id);
                    }
                let mut need_lookup = false;
                match cached_users.get(&sender) {
                    Some(user) => {
                        match user.send_channel_sink.send(rfp.clone().into()).await {
                            Ok(_) => {}
                            // Recipient may have logged off and back on - invalidating the
                            // cached handle.
                            Err(_) => {
                                cached_users.remove(&sender);
                                need_lookup = true;
                            }
                        };
                    }
                    None => {
                        need_lookup = true;
                    }
                };

                if need_lookup {
                    let their_new_handle: Option<ClientHandle>;
                    {
                        let master_users_ro;
                        match master_users.read() {
                            Ok(ro) => master_users_ro = ro,
                            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                        }
                        their_new_handle = match master_users_ro.get(&sender) {
                            Some(h) => {
                                cached_users.insert(sender.clone(), h.clone());
                                Some(h.clone())
                            }
                            None => None,
                        }
                    }
                    match their_new_handle {
                        Some(handle) => {
                            handle.send_channel_sink.send(rfp.into()).await?;
                        }
                        None => {
                            let mut reply = QueryUserPacket::new(&sender)?;
                            reply.set_offline();
                            channel_sink.send(reply.into()).await?;
                        }
                    };
                }

                //disconnect the offending user if they sent us a garbage rejection
                if !valid_transfer{
                    let error_reply = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER).unwrap();
                    channel_sink.send(error_reply.into()).await?;
                    break; //Closes connection to the offending user.
                }



                }
            IrcKind::IRC_KIND_FILE_TRANSFER => {}
            _ => {}
        }
    }
    Ok(ret_string.into())
}

/*async fn attemptForwardWithQueryReply<'a>(outgoing: &dyn IrcPacket, target: String,
    channel_sink: &mut mpsc::Sender<irclib::SyncSendPack>,
    master_rooms: &mut Arc<RwLock<HashMap<String, RoomHandle>>>,
    master_users: &mut Arc<RwLock<HashMap<String, ClientHandle>>>,
    cached_users: &mut HashMap<String, ClientHandle>
) -> Result<'a, ()>{
}*/

async fn make_room<'a>(
    room_name: String,
    master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
    master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
) -> Result<'a, RoomHandle> {
    let (join_channel_sink, mut join_channel_source) = mpsc::channel::<ClientHandle>(32);
    let (post_channel_sink, mut post_channel_source) = mpsc::channel::<SyncSendPack>(64);
    let (leave_channel_sink, mut leave_channel_source) = mpsc::channel::<String>(32);
    let rn1 = room_name.clone();
    let p1 = post_channel_sink.clone();
    let u1 = master_users.clone();
    let r1 = master_rooms.clone();

    tokio::spawn(room_lifecycle(
        rn1,
        join_channel_source,
        p1,
        post_channel_source,
        leave_channel_source,
        u1,
        r1,
    ));

    let new_room_handle = RoomHandle {
        join_channel_sink: join_channel_sink,
        post_channel_sink: post_channel_sink,
        leave_channel_sink: leave_channel_sink,
    };

    //Add this room to the master list and inform all users
    let mut outgoing = RoomListingPacket::new()?;
    {
        let mut master_rooms_rw;
        match master_rooms.write() {
            Ok(rw) => master_rooms_rw = rw,
            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
        }
        master_rooms_rw.insert(room_name, new_room_handle.clone());
        for (key, _) in master_rooms_rw.iter() {
            println!("New room opened: {}", key);
            outgoing.push(key)?;
        }
    }

    let mut clients_to_notify: Vec<mpsc::Sender<SyncSendPack>> = Vec::new();

    {
        let master_users_ro;
        match master_users.read() {
            Ok(ro) => master_users_ro = ro,
            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
        }
        for (_, client) in master_users_ro.iter() {
            clients_to_notify.push(client.send_channel_sink.clone());
        }
    }

    for client in &clients_to_notify {
        client.send(outgoing.clone().into()).await?;
    }

    Ok(new_room_handle)
}

async fn room_lifecycle<'a>(
    room_name: String,
    mut join_source: mpsc::Receiver<ClientHandle>,
    mut post_sink: mpsc::Sender<SyncSendPack>,
    mut post_source: mpsc::Receiver<SyncSendPack>,
    mut leave_source: mpsc::Receiver<String>,
    master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
    master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
) -> Result<'a, ()> {
    let users_in_room: Arc<RwLock<HashMap<String, ClientHandle>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let u1 = users_in_room.clone();
    let u2 = users_in_room.clone();
    let u3 = users_in_room.clone();

    let p1 = post_sink.clone();
    let p2 = post_sink.clone();

    let rn1 = room_name.clone();
    let rn2 = room_name.clone();
    let rn3 = room_name.clone();

    tokio::select! {
        _ = users_entering_room(rn1, join_source, u1, p1) => {},
        _ = users_leaving_room(rn2, leave_source, u2, p2) => {},
        _ = messages_posting_to_room(rn3, post_source, u3) => {},
    }

    println!("Closing room: {}", &room_name);

    //Remove this room from the master list and inform all users
    let mut outgoing = RoomListingPacket::new()?;
    {
        let mut master_rooms_rw;
        match master_rooms.write() {
            Ok(rw) => master_rooms_rw = rw,
            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
        }
        master_rooms_rw.remove(&room_name);
        for (key, _) in master_rooms_rw.iter() {
            outgoing.push(key)?;
        }
    }

    let mut clients_to_notify: Vec<mpsc::Sender<SyncSendPack>> = Vec::new();

    {
        let master_users_ro;
        match master_users.read() {
            Ok(ro) => master_users_ro = ro,
            Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
        }
        for (_, client) in master_users_ro.iter() {
            clients_to_notify.push(client.send_channel_sink.clone());
        }
    }

    for client in &clients_to_notify {
        client.send(outgoing.clone().into()).await?;
    }
    Ok(())
}

async fn users_entering_room<'a>(
    room_name: String,
    mut join_source: mpsc::Receiver<ClientHandle>,
    users_in_room: Arc<RwLock<HashMap<String, ClientHandle>>>,
    mut post_sink: mpsc::Sender<SyncSendPack>,
) -> Result<'a, String> {
    while let Some(entering_user) = join_source.recv().await {
        println!("{} enters {}", entering_user.name, room_name);
        let mut clients_to_notify: Vec<mpsc::Sender<SyncSendPack>> = Vec::new();
        let mut outgoing = UserListingPacket::new()?;
        outgoing.set_room(&room_name)?;
        let mut user_is_new = false;

        {
            let mut users_in_room_rw;
            match users_in_room.write() {
                Ok(rw) => users_in_room_rw = rw,
                Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
            }
            if !users_in_room_rw.contains_key(&entering_user.name) {
                user_is_new = true;
                users_in_room_rw.insert(entering_user.name.clone(), entering_user.clone());
                for (key, handle) in users_in_room_rw.iter() {
                    outgoing.push(key)?;
                    clients_to_notify.push(handle.send_channel_sink.clone());
                }
            }
        }

        if user_is_new {
            for client in &clients_to_notify {
                client.send(outgoing.clone().into()).await?;
            }
        }
    }
    Ok("no more users may enter".to_string())
}

async fn users_leaving_room<'a>(
    room_name: String,
    mut leave_source: mpsc::Receiver<String>,
    users_in_room: Arc<RwLock<HashMap<String, ClientHandle>>>,
    mut post_sink: mpsc::Sender<SyncSendPack>,
) -> Result<'a, String> {
    while let Some(leaving_user) = leave_source.recv().await {
        println!("{} leaves {}", leaving_user, room_name);

        let mut clients_to_notify: Vec<mpsc::Sender<SyncSendPack>> = Vec::new();
        let mut outgoing = UserListingPacket::new()?;
        outgoing.set_room(&room_name)?;
        let mut user_removed = false;
        let mut have_users = true;

        {
            let mut users_in_room_rw;
            match users_in_room.write() {
                Ok(rw) => users_in_room_rw = rw,
                Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
            }

            if users_in_room_rw.contains_key(&leaving_user) {
                users_in_room_rw.remove(&leaving_user);
                user_removed = true;
                for (key, handle) in users_in_room_rw.iter() {
                    outgoing.push(key)?;
                    clients_to_notify.push(handle.send_channel_sink.clone());
                }
                have_users = users_in_room_rw.len() > 0;
            }
        }

        if user_removed {
            for client in &clients_to_notify {
                client.send(outgoing.clone().into()).await?;
            }
        }

        if !have_users {
            break;
        }
    }
    Ok("I'm not trapped in here with you... you're trapped in here with me!".to_string())
}

async fn messages_posting_to_room<'a>(
    room_name: String,
    mut post_source: mpsc::Receiver<SyncSendPack>,
    users_in_room: Arc<RwLock<HashMap<String, ClientHandle>>>,
) -> Result<'a, ()> {
    while let Some(message_packed) = post_source.recv().await {
        if message_packed.contained_kind == IrcKind::IRC_KIND_POST_MESSAGE {
            let mut clients_to_notify: Vec<mpsc::Sender<SyncSendPack>> = Vec::new();
            let mut outgoing = message_packed.pmp.unwrap();

            {
                let users_in_room_ro;
                match users_in_room.read() {
                    Ok(ro) => users_in_room_ro = ro,
                    Err(e) => return Err(IrcError::PoisonedErr(format!("{}", e))),
                }
                for (_, handle) in users_in_room_ro.iter() {
                    clients_to_notify.push(handle.send_channel_sink.clone());
                }
            }

            for client in &clients_to_notify {
                client.send(outgoing.clone().into()).await?;
            }
        }
    }
    Ok(())
}
