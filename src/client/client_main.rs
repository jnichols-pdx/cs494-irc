#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::*;

use num_enum::FromPrimitive;
use std::env;
use std::error::Error;
use std::io::{stderr, Write};
use std::fs::File;
use std::path::Path;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use cursive::Cursive;
//use cursive::CursiveRunnable;
use crate::uilib::*;
use cursive::view::*;
use cursive::views::TextView;
use cursive::views::*;
use cursive_tabs::TabPanel;
use std::thread;

#[tokio::main]
async fn main() -> Result<'static, ()> {
    let running = Arc::new(AtomicBool::new(true));
    let r1 = running.clone();
    let r2 = running.clone();
    let r3 = running.clone();
    ctrlc::set_handler(move || {
        r1.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let ui_active = Arc::new(AtomicBool::new(true));
    let u1 = ui_active.clone();

    let found_pulse = Arc::new(AtomicBool::new(true));
    let fp = found_pulse.clone();

    let mut arg_list = env::args().skip(1);
    let my_name = arg_list.next().unwrap();

    let host;
    if arg_list.len() > 0 {
        host = arg_list.next().unwrap();
    } else {
        host = "192.168.2.5:17734".to_string();
    }

    let mut con = TcpStream::connect(host).await?;
    let my_ident = NewClientPacket::new(&my_name)?;
    con.write(&my_ident.as_bytes()).await?;

    //thread to thread communication channel for incoming packets to the UI
    let (tx_to_responder, mut ui_rx) = mpsc::channel::<SyncSendPack>(32);

    //thread to thread communication channel for outgoing packets to the Server
    let (tx_to_server, mut outgoing_rx) = mpsc::channel::<SyncSendPack>(32);
    let tx2 = tx_to_server.clone();
    let tx3 = tx_to_server.clone();
    let tx4 = tx_to_server.clone();
    let tx5 = tx_to_server.clone();
    let tx6 = tx_to_server.clone();

    //Oneshot to retrieve a sender to the Cursive UI's callback channel from the kickstart
    //function.
    let (tx_cb_to_main, mut rx_cb_from_kickstart) = oneshot::channel::<cursive::CbSink>();

    //Start the console UI in it's own OS thread (doesn't play nice with tokio's
    //task management green threads).
    let ui_thread = thread::spawn(move || ui_kickstart(r3, tx_cb_to_main, tx2, u1));

    //Retrieve a handle to the UI's callback endpoint, so we may tell it to react to incoming
    //packets.
    let cb_handle;
    match rx_cb_from_kickstart.await {
        Ok(v) => cb_handle = v,
        Err(e) => {
            println!("Got Err instead of handle{:?}", e);
            return Err(IrcError::PacketMismatch());
        }
    }
    let cb1 = cb_handle.clone();

    //Split the TcpStream into reader and writer, pass each to their own asynchronous task
    let (tcp_in, tcp_out) = con.into_split();

    //Spawn asynchronous tokio tasks to watch for shutdown triggers, receive messages from, and
    //send messages to the IRC server.
    let responder_task = tokio::spawn(responder(cb_handle, ui_rx, tx5));
    let send_task = tokio::spawn(writer(tcp_out, outgoing_rx));
    let heartbeat_task = tokio::spawn(pulse(tx3));
    let read_task = tokio::spawn(reader(tcp_in, tx_to_responder, fp, tx4));
    let watchdog_task = tokio::spawn(pulse_monitor(found_pulse));
    let stop_task = tokio::spawn(shutdown_monitor(r2, tx6));

    let first_room_listing = ListRoomsPacket::new()?;
    tx_to_server.send(first_room_listing.into()).await?;

    let offline_message; //Message to show user when the server goes down.

    //Polls the tokio tasks to keep them operating, ends all tasks when one completes
    //stop_task will complete when the Cursive UI event loop exits (explicit quit, or catches ctrl-c)
    //    or if our direct ctrl-c handler gets a ctrl-c signal that evaded Cursive's runtime.
    //read_task will complete when the incoming network connection from the server closes, or we
    //    recieve a ServerDepart message indicating the server will close.
    //responder_task may complete when read_task drops the communication channel they share
    //send_task will complete with the outgoing network connection to the server closes.
    //watchdog_task will complete if the server fails to send heartbeats for 30 seconds.
    tokio::select! {
        out = read_task => {offline_message = format!("{}",out??);},
        out = responder_task => {offline_message = format!("Response: {:?}",out?);},
        _ = stop_task => {offline_message = "User asked to quit.".into();},
        _ = send_task => {offline_message = "Outgoing channel died...".into();},
        _ = heartbeat_task => {offline_message = "Internal Error (client keepalive failed)".into();},
        _ = watchdog_task => {offline_message = "Server hasn't responded in 30 seconds.".into();},
    }

    if ui_active.load(Ordering::SeqCst) {
        //UI thread hasn't exited, but we know we want to quit.
        //Attempt to have the UI give the user an error prompt about the reason we are now
        //disconnected from the server.
        let could_prompt = cb1.send(Box::new(move |s: &mut cursive::Cursive| {
            s.add_layer(
                Dialog::text(offline_message)
                    .title("Disconnected")
                    .button("Quit", |s| s.quit()),
            );
        }));

        match could_prompt {
            Ok(_) => {
                //UI was still responsive - wait for the user to acknowledge the disconnect dialog prompt.
                let mut wait_period = time::interval(Duration::from_millis(100));
                loop {
                    wait_period.tick().await;
                    if !ui_active.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
            Err(_) => {} //UI wasn't responsive (locked up?) - fall through and exit program.
        }
    }

    Ok(())
}

fn ui_kickstart(
    running: Arc<AtomicBool>,
    tx_return_cb_handle: oneshot::Sender<cursive::CbSink>,
    tx_packet_out: mpsc::Sender<irclib::SyncSendPack>,
    ui_active: Arc<AtomicBool>,
) {
    let mut siv = cursive::default();
    let cb = siv.cb_sink().clone();
    tx_return_cb_handle
        .send(cb)
        .expect("Couldn't pass back cb handle");

    //Global callback to exit the program, will need to change this to pass a message to our core
    //program code to do a graceful disconnect.
    siv.add_global_callback(cursive::event::Event::CtrlChar('q'), |s| s.quit());

    let mut panel = TabPanel::new();
    let tx1 = tx_packet_out.clone();
    panel.add_tab(make_rooms_page(tx1));

    let panelv = panel
        .with_name("TABS__________________________32+")
        .full_screen();

    siv.add_fullscreen_layer(panelv);

    //Callbacks to capture ctrl-right and ctrl-left and switch tabs accordingly.
    siv.add_global_callback(
        cursive::event::Event::Ctrl(cursive::event::Key::Left),
        switch_prev,
    );
    siv.add_global_callback(
        cursive::event::Event::Ctrl(cursive::event::Key::Right),
        switch_next,
    );

    siv.run();

    ui_active.store(false, Ordering::SeqCst);
    running.store(false, Ordering::SeqCst);
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

async fn responder(
    cb: cursive::CbSink,
    mut rx_from_main: mpsc::Receiver<SyncSendPack>,
    tx_packet_out: mpsc::Sender<irclib::SyncSendPack>,
) {
    while let Some(packet) = rx_from_main.recv().await {
        match packet.contained_kind {
            IrcKind::IRC_KIND_ROOM_LISTING => {
                let rlp = packet.rlp.unwrap();
                cb.send(Box::new(move |s: &mut cursive::Cursive| {
                    s.call_on_name(
                        "Rooms----------------------select",
                        |select: &mut SelectView<String>| {
                            select.clear();
                            select.add_all_str(rlp.rooms.into_iter());
                        },
                    );
                }))
                .unwrap();
            }
            IrcKind::IRC_KIND_USER_LISTING => {
                let ulp = packet.ulp.unwrap();
                let room_name = ulp.room.to_owned();
                let txr = tx_packet_out.clone();
                cb.send(Box::new(move |s: &mut cursive::Cursive| {
                    match s.find_name::<TextView>(
                        format!("{}--------------------------people", ulp.room).as_str(),
                    ) {
                        Some(mut list) => {
                            list.set_content("");
                        }
                        None => {
                            s.call_on_name(
                                "TABS__________________________32+",
                                |tab_controller: &mut TabPanel| {
                                    tab_controller.add_tab(make_room(
                                        room_name.into(),
                                        "".into(),
                                        txr,
                                    ));
                                },
                            );
                        }
                    };
                    s.call_on_name(
                        format!("{}--------------------------people", ulp.room).as_str(),
                        |users_list: &mut TextView| {
                            for user in ulp.users {
                                users_list.append(format!("{}\n", user));
                            }
                        },
                    );
                }))
                .unwrap();
            }
            IrcKind::IRC_KIND_QUERY_USER => {
                let qup = packet.qup.unwrap();
                let user_name = qup.user_name.to_owned();
                let status = qup.status.to_owned();
                cb.send(Box::new(move |s: &mut cursive::Cursive| {
                    s.add_layer(
                        Dialog::text(format!("{} is {}.", user_name, status))
                            .title("User Status")
                            .dismiss_button("Noted"),
                    );
                }))
                .unwrap();
            }
            IrcKind::IRC_KIND_POST_MESSAGE => {
                let pmp = packet.pmp.unwrap();
                let room_name = pmp.room.to_owned();
                let sender = pmp.sender.to_owned();
                let message = pmp.get_message().to_owned();
                cb.send(Box::new(move |s: &mut cursive::Cursive| {
                    s.call_on_name(
                        format!("{}-------------------------content", room_name).as_str(),
                        |content: &mut TextView| {
                            content.append(format!("{}: {}\n", sender, message));
                        },
                    );
                }))
                .unwrap();
            }
            IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                let dmp = packet.dmp.unwrap();
                let with_who = dmp.target.to_owned();
                let txr = tx_packet_out.clone();
                cb.send(Box::new(move |s: &mut cursive::Cursive| {
                    match s.find_name::<TextView>(
                        format!("DM:{}-------------------------content", dmp.target).as_str(),
                    ) {
                        Some(mut convo) => {
                            convo.append(format!("{}: {}\n", with_who, dmp.get_message()));
                        }
                        None => {
                            let with2 = with_who.clone();
                            s.call_on_name(
                                "TABS__________________________32+",
                                |tab_controller: &mut TabPanel| {
                                    tab_controller.add_tab(make_dm_room(
                                        with2.into(),
                                        format!("{}: {}\n", with_who, dmp.get_message()).into(),
                                        txr,
                                    ));
                                },
                            );
                        }
                    };
                }))
                .unwrap();
            }
            IrcKind::IRC_KIND_OFFER_FILE => {}
            IrcKind::IRC_KIND_ACCEPT_FILE => {}
            IrcKind::IRC_KIND_REJECT_FILE => {}
            IrcKind::IRC_KIND_FILE_TRANSFER => {}
            _ => {}
        }
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

async fn shutdown_monitor<'a>(
    running: Arc<AtomicBool>,
    tx_packet_out: mpsc::Sender<irclib::SyncSendPack>,
) -> Result<'a, ()> {
    let mut wait_period = time::interval(Duration::from_millis(100));
    loop {
        wait_period.tick().await;
        if !running.load(Ordering::SeqCst) {
            //we've been asked to close - so send some cleanup packets!
            let outgoing = ClientDepartsPacket::new(&"Client going outside!".to_string())
                .expect("Error packets should be infallible on creation");
            tx_packet_out.send(outgoing.into()).await?;
            break;
        }
    }
    Ok(())
}

async fn writer<'a>(
    mut con: tokio::net::tcp::OwnedWriteHalf,
    mut rx_packets_to_send: mpsc::Receiver<SyncSendPack>,
) -> Result<'a, ()> {
    let mut bytes_to_go;
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
            }
            _ => {
                println!("Can't send Unknown type packet!");
                continue;
            }
        }
        con.write(&bytes_to_go).await?;
        con.flush().await?;
    }
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
            let mut buffer = vec![0u8; msg_len + 5];
            con.read_exact(&mut buffer).await?;
            {
            //let bytes_read = con.read(&mut buffer).await?;
            //if bytes_read == msg_len + 5 {
                let kind_raw = IrcKind::from(buffer[0]);
                match kind_raw {
                    IrcKind::IRC_KIND_ERR => {
                        let my_error = ErrorPacket::from_bytes(&buffer[0..6])?;
                        match my_error.error_code {
                            IrcErrCode::IRC_ERR_UNKNOWN => {
                                ret_string =
                                    "Bogus! Server's confused (we received Error: Unknown)".into();
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
                    IrcKind::IRC_KIND_NEW_CLIENT => { /*println!("Got New client packet...?");*/ }
                    IrcKind::IRC_KIND_HEARTBEAT => {
                        found_pulse.store(true, Ordering::SeqCst);
                    }
                    IrcKind::IRC_KIND_ENTER_ROOM => { /*println!("Got enter room packet...?");*/ }
                    IrcKind::IRC_KIND_LEAVE_ROOM => { /*println!("Got leave room packet...?");*/ }
                    IrcKind::IRC_KIND_LIST_ROOMS => { /*println!("Got list rooms packet...?");*/ }
                    IrcKind::IRC_KIND_ROOM_LISTING => {
                        let room_list = RoomListingPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(room_list.into()).await?;
                    }
                    IrcKind::IRC_KIND_USER_LISTING => {
                        let user_list = UserListingPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(user_list.into()).await?;
                    }
                    IrcKind::IRC_KIND_QUERY_USER => {
                        let query_result = QueryUserPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(query_result.into()).await?;
                    }
                    IrcKind::IRC_KIND_SEND_MESSAGE => { /*println!("Got send message packet...?");*/
                    }
                    IrcKind::IRC_KIND_BROADCAST_MESSAGE => { /*println!("Got broadcast message packet...?");*/
                    }
                    IrcKind::IRC_KIND_POST_MESSAGE => {
                        let new_message = PostMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_message.into()).await?;
                    }

                    IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                        let new_direct = DirectMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_direct.into()).await?;
                    }
                    IrcKind::IRC_KIND_OFFER_FILE => {
                        // println!("Got offer file packet.");
                    }
                    IrcKind::IRC_KIND_ACCEPT_FILE => {
                        // println!("Got accept file packet.");
                    }
                    IrcKind::IRC_KIND_REJECT_FILE => {
                        // println!("Got reject file packet.");
                    }
                    IrcKind::IRC_KIND_FILE_TRANSFER => {
                        //  println!("Got file transfer packet.");
                    }
                    IrcKind::IRC_KIND_CLIENT_DEPARTS => { /* println!("Got client departs packet...?");*/
                    }
                    IrcKind::IRC_KIND_SERVER_DEPARTS => {
                        let server_leaving = ServerDepartsPacket::from_bytes(&buffer[..])?;
                        ret_string = format!(
                            "Server disconnected with this message: \"{}\"",
                            server_leaving.get_message()
                        );
                        break;
                    }
                    _ => {
                        let _ = writeln!(
                            stderr(),
                            "Error: Unknown packet recieved:\n{:?}\n",
                            //&buffer[0..bytes_read]
                            &buffer
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
                ret_string = "Read connection to server has closed.".into();
                break;
            }
        }
    }
    Ok(ret_string.into())
}

#[path = "curs.rs"]
mod uilib;  //Include the UI specific code kept in a separate file.
