#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::{*};

use tokio::net::{TcpListener, TcpStream};
use bytes::{Bytes, BytesMut, Buf, BufMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::{self, Duration, sleep};
use tokio::sync::{mpsc, oneshot};

use std::io::{ErrorKind};
use std::collections::HashMap;
use std::io::{Write,stderr};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,RwLock};
use ctrlc;


#[derive(Clone, Debug)]
pub struct ClientHandle {
    pub name: String,
    pub send_channel_sink: mpsc::Sender<SyncSendPack>,
}
/*
pub struct Client<'a,'b> {
    pub name: String,
    pub found_pulse: Arc<AtomicBool>,
    //pub connection: TcpStream,
    pub tcp_output: tokio::net::tcp::OwnedWriteHalf,
    pub tcp_input: tokio::net::tcp::OwnedReadHalf,
    pub send_channel_source: mpsc::Receiver<SyncSendPack>,
    pub send_channel_sink: mpsc::Sender<SynSendPack>,
    pub cached_rooms: HashMap<String, <RoomHandle>>,
    pub cached_users: HashMap<String, <RoomHandle>>,

    pub master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>,
    pub master_users: Arc<RwLock<HashMap<String, ClientHandle>>>,
}
*/
#[derive(Clone, Debug)]
pub struct RoomHandle {
    pub join_channel_sink: mpsc::Sender<ClientHandle>,
    pub post_channel_sink: mpsc::Sender<SyncSendPack>,
    pub leave_channel_sink: mpsc::Sender<String>,
}
/*
pub struct Room<'a,'b> {
    pub name: String,
    pub post_channel_source: mpsc::Receiver<SyncSendPack>,
    pub post_channel_sink: mpsc::Sender<SyncSendPack>,
    pub join_channel_source: mpsc::Receiver<ClientShort>,
    pub join_channel_sink: mpsc::Sender<ClientHandle>,
    pub leave_channel_source: mpsc::Receiver<String>,
    pub leave_channel_sink: mpsc::Sender<String>,
    pub user_handles: Arc<RwLock<HashMap<String, mpsc::Sender<SyncSendPack>>>>,
}
*/


#[tokio::main]
async fn main() -> Result<'static, ()> {
    println!("Hello, world! [server]");
    let running = Arc::new(AtomicBool::new(true));
    let r1 = running.clone();
    let r2 = running.clone();
    ctrlc::set_handler(move || {
        r1.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");




    let listener = TcpListener::bind("0.0.0.0:17734").await?;

    let master_rooms =  Arc::new(RwLock::new(HashMap::new()));//<String, RoomHandle>
    let master_users =  Arc::new(RwLock::new(HashMap::new()));//<String, ClientHandle>

    let mrc = master_rooms.clone();
    let muc = master_users.clone();
    let listener_task= tokio::spawn(new_connections(listener, mrc, muc));
    let stop_task = tokio::spawn(shutdown_monitor(r2));

    //let offline_message;
    //blocks until one of the tasks listed returns
    tokio::select!{
        out = listener_task => {},//offline_message = format!("Response: {:?}",out?);},
        _ = stop_task => {},//offline_message = "User asked to quit.".into();},
    }
    
    let final_users = master_users.write().unwrap(); //grab exclusive access to user to say our goodbyes.

    //println!("about to exit");
    for (_,client_handle) in final_users.iter() { 
        let outgoing = ServerDepartsPacket::new(&"Server going down for maintenance.".to_string())
            .expect("Server closing anyway.");
        client_handle.send_channel_sink.send(outgoing.into()).await?;
    }
    sleep(Duration::from_millis(1000)).await;
    Ok(())
}

async fn shutdown_monitor(running: Arc<AtomicBool>){
    let mut wait_period = time::interval(Duration::from_millis(100));
    loop {
        wait_period.tick().await;
        if !running.load(Ordering::SeqCst) {
            println!("Caught SIGINT, shutting down.");
            //ctrl-c was pressed, break to signal we should shutdown.
            //println!("Detected ctl-c");
            break;
        }
    }
}

async fn new_connections<'a>(listener: TcpListener, master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>, master_users: Arc<RwLock<HashMap<String, ClientHandle>>>) -> Result<'a, ()> {

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
                match  kind_raw {
                    IrcKind::IRC_KIND_NEW_CLIENT => {
                        let new_client = NewClientPacket::from_bytes(&buffer)?;    
                        let master_users_copy = master_users.clone();
                        let master_rooms_copy = master_rooms.clone();
                        let mut should_reject;
                        {
                            let mut master_users_open = master_users.read().unwrap();
                            should_reject =  master_users_open.contains_key(&new_client.chat_name);
                        } 
                        if should_reject {
                            println!("Rejecting duplicate user: {}", new_client.chat_name);
                            socket.write(&ErrorPacket::new(IrcErrCode::IRC_ERR_NAME_IN_USE)?.as_bytes()).await?;
                            socket.shutdown().await?;
                        } else {
                            println!("New client connected: '{}'", new_client.chat_name);
                            //Spin up a new user
                            socket.set_nodelay(true).expect("Unable to set delay false");
                            let (channel_sink, mut channel_source) = mpsc::channel::<SyncSendPack>(64);
                            //let handle_sink = channel_sink.clone();

                            let client_name = new_client.chat_name.clone();
                            let new_client_handle1 = ClientHandle {
                                send_channel_sink: channel_sink,
                                name: new_client.chat_name,
                            };


                            let new_client_handle2 = new_client_handle1.clone();

                            {
                                let mut master_users_ours = master_users.write().unwrap();
                                master_users_ours.insert(client_name, new_client_handle1);
                            }
                            tokio::spawn(client_lifecycle(socket, master_rooms_copy, master_users_copy, new_client_handle2, channel_source));
                        }
    
                    }
                    _ => {
                            let _ =  writeln!(stderr(),"Error: Expected New Client Packet for new connection, received:\n{:?}\n",&buffer[0..bytes_read]);
                            let error_notice = ErrorPacket::new(IrcErrCode::IRC_ERR_UNKNOWN)
                                .expect("Error packets should be infallible on creation");
                            socket.write(&error_notice.as_bytes()).await?;
                            socket.shutdown().await?;
                    },
                }
            }
        }
    }
}



async fn client_lifecycle(mut socket: TcpStream, master_rooms: Arc<RwLock<HashMap<String, RoomHandle>>>, master_users: Arc<RwLock<HashMap<String, ClientHandle>>>, mut our_handle: ClientHandle, mut channel_source: mpsc::Receiver<SyncSendPack>){
    //Split the TcpStream into reader and writer, pass each to their own asynchronous task
    let (tcp_in, tcp_out) = socket.into_split();
    let client_name = our_handle.name;
    let channel_sink = our_handle.send_channel_sink;
    let sink1 = channel_sink.clone();
    let found_pulse = Arc::new(AtomicBool::new(true));
    let fp = found_pulse.clone();

    /*let send_task = tokio::spawn(writer(tcp_out, channel_source));
    let heartbeat_task = tokio::spawn(pulse(sink1));
    let watchdog_task = tokio::spawn(pulse_monitor(found_pulse));*/

    let offline_message: &str;
    tokio::select!{
        //out = read_task => {offline_message = format!("{}",out??);},
        //out = responder_task => {offline_message = format!("Response: {:?}",out?);},
        /*_ = send_task => {offline_message = "Downstream connection ended.".into();},
        _ = heartbeat_task => {offline_message = "Internal Error (server keepalive failed).".into();},
        _ = watchdog_task => {offline_message = "No heartbeat responded in 30 seconds.".into();},*/
        _ = writer(tcp_out, channel_source) => {offline_message = "Downstream connection ended.".into();},
        _ = pulse(sink1) => {offline_message = "Internal Error (server keepalive failed).".into();},
        _ = pulse_monitor(found_pulse) => {offline_message = "No heartbeat responded in 30 seconds.".into();},
    }

    println!("Client '{}' ejected: {}",&client_name, &offline_message);
    {
        let mut master_users_ours = master_users.write().unwrap();
        master_users_ours.remove(&client_name);
    }
    

}

async fn pulse<'a>(tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()>
{
    let mut wait_period = time::interval(Duration::from_millis(5000));
    loop {
        wait_period.tick().await;
        let heartbeat = HeartbeatPacket::new().expect("Heartbeat packets should be infallible on creation");
        tx_packet_out.send(heartbeat.into()).await?;
    }
}

async fn pulse_monitor<'a>(found_pulse: Arc<AtomicBool>)  -> Result<'a,()>
{
    let mut seconds_since_heartbeat = 0 as u8;
    let mut wait_period = time::interval(Duration::from_millis(1000));
    loop {
        wait_period.tick().await;
        if found_pulse.load(Ordering::SeqCst) {
            seconds_since_heartbeat = 0;
            found_pulse.store(false, Ordering::SeqCst);
        }else {
            if seconds_since_heartbeat >= 30 {
                break;
            }
            seconds_since_heartbeat +=1;
        }
    }

Ok(())
}

async fn writer<'a>(mut con: tokio::net::tcp::OwnedWriteHalf, mut rx_packets_to_send: mpsc::Receiver<SyncSendPack>) -> Result<'a,()> {
    let mut bytes_to_go;
    let mut should_stop = false;
    while let Some(sync_send_packet) = rx_packets_to_send.recv().await {
        //println!("send me packets!");
        match sync_send_packet.contained_kind {
            IrcKind::IRC_KIND_ERR => {bytes_to_go = sync_send_packet.errp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_NEW_CLIENT => {bytes_to_go = sync_send_packet.ncp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_HEARTBEAT => {bytes_to_go = sync_send_packet.hbp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_ENTER_ROOM => {bytes_to_go = sync_send_packet.erp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_LEAVE_ROOM => {bytes_to_go = sync_send_packet.lrp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_LIST_ROOMS => {bytes_to_go = sync_send_packet.lip.unwrap().as_bytes();}
            IrcKind::IRC_KIND_ROOM_LISTING => {bytes_to_go = sync_send_packet.rlp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_USER_LISTING => {bytes_to_go = sync_send_packet.ulp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_QUERY_USER => {bytes_to_go = sync_send_packet.qup.unwrap().as_bytes();}
            IrcKind::IRC_KIND_SEND_MESSAGE => {bytes_to_go = sync_send_packet.smp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_BROADCAST_MESSAGE => { bytes_to_go = sync_send_packet.bmp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_POST_MESSAGE => {bytes_to_go = sync_send_packet.pmp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_DIRECT_MESSAGE => {bytes_to_go = sync_send_packet.dmp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_OFFER_FILE => {bytes_to_go = sync_send_packet.ofp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_ACCEPT_FILE => {bytes_to_go = sync_send_packet.afp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_REJECT_FILE => {bytes_to_go = sync_send_packet.rfp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_FILE_TRANSFER => {bytes_to_go = sync_send_packet.ftp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_CLIENT_DEPARTS => {bytes_to_go = sync_send_packet.cdp.unwrap().as_bytes();}
            IrcKind::IRC_KIND_SERVER_DEPARTS => {bytes_to_go = sync_send_packet.sdp.unwrap().as_bytes(); should_stop = true;}
            _ => {println!("Can't send Unknown type packet!");continue;},
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

async fn reader<'a>(mut con: tokio::net::tcp::OwnedReadHalf, tx_to_responder: mpsc::Sender<SyncSendPack>, found_pulse: Arc<AtomicBool>,tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, String> {
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
                match  kind_raw {
                    IrcKind::IRC_KIND_ERR => {
                        let my_error = ErrorPacket::from_bytes(&buffer[0..6])?;
                        match my_error.error_code {
                            IrcErrCode::IRC_ERR_UNKNOWN => { ret_string = "Bogus! Server's confused (we received Error: Unknown)".into();},
                            IrcErrCode::IRC_ERR_ILLEGAL_KIND => { ret_string = "Bogus! Illegal Kind!".into();},
                            IrcErrCode::IRC_ERR_ILLEGAL_LENGTH => { ret_string = "Bogus! Illegal Length!".into();},
                            IrcErrCode::IRC_ERR_NAME_IN_USE => { ret_string = "Bogus! That name's taken!".into();},
                            IrcErrCode::IRC_ERR_ILLEGAL_NAME => { ret_string = "Bogus! Illegal Name!".into();},
                            IrcErrCode::IRC_ERR_ILLEGAL_MESSAGE => { ret_string = "Bogus! Illegal Message!".into();},
                            IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER => { ret_string = "Bogus! Illegal Transfer!".into();},
                            IrcErrCode::IRC_ERR_TOO_MANY_USERS => { ret_string = "Bogus! Slashdoted! (too many users)".into();},
                            IrcErrCode::IRC_ERR_TOO_MANY_ROOMS => { ret_string = "Bogus! Too Many Rooms!".into();},
                            _ => (),
                        }
                        break;
                    },
                    IrcKind::IRC_KIND_NEW_CLIENT => {/*println!("Got New client packet...?");*/},
                    IrcKind::IRC_KIND_HEARTBEAT => {
                        found_pulse.store(true, Ordering::SeqCst);
                    },
                    IrcKind::IRC_KIND_ENTER_ROOM => {/*println!("Got enter room packet...?");*/},
                    IrcKind::IRC_KIND_LEAVE_ROOM => {/*println!("Got leave room packet...?");*/},
                    IrcKind::IRC_KIND_LIST_ROOMS => {/*println!("Got list rooms packet...?");*/},
                    IrcKind::IRC_KIND_ROOM_LISTING => {
                        let room_list = RoomListingPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(room_list.into()).await?;
                    },
                    IrcKind::IRC_KIND_USER_LISTING => {
                        let user_list = UserListingPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(user_list.into()).await?;
                    },
                    IrcKind::IRC_KIND_QUERY_USER => {
                        let query_result = QueryUserPacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(query_result.into()).await?;
                    },
                    IrcKind::IRC_KIND_SEND_MESSAGE => {/*println!("Got send message packet...?");*/},
                    IrcKind::IRC_KIND_BROADCAST_MESSAGE => {/*println!("Got broadcast message packet...?");*/},
                    IrcKind::IRC_KIND_POST_MESSAGE => {
                        let new_message = PostMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_message.into()).await?;
                    },

                    IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                        let new_direct = DirectMessagePacket::from_bytes(&buffer[..])?;
                        tx_to_responder.send(new_direct.into()).await?;
                    },
                    IrcKind::IRC_KIND_OFFER_FILE => {
                        //println!("Got offer file packet.");
                    },
                    IrcKind::IRC_KIND_ACCEPT_FILE => {
                       // println!("Got accept file packet.");
                    },
                    IrcKind::IRC_KIND_REJECT_FILE => {
                       // println!("Got reject file packet.");
                    },
                    IrcKind::IRC_KIND_FILE_TRANSFER => {
                      //  println!("Got file transfer packet.");
                    },
                    IrcKind::IRC_KIND_CLIENT_DEPARTS => { println!("Got client departs packet...?"); },
                    IrcKind::IRC_KIND_SERVER_DEPARTS => {
                        let  server_leaving = ServerDepartsPacket::from_bytes(&buffer[..])?;
                        ret_string = format!("Server disconnected with this message: \"{}\"", server_leaving.get_message());
                        break;
                    },
                    _ => {
                            let _ =  writeln!(stderr(),"Error: Unknown packet recieved:\n{:?}\n",&buffer[0..bytes_read]);
                            let error_notice = ErrorPacket::new(IrcErrCode::IRC_ERR_UNKNOWN)
                                .expect("Error packets should be infallible on creation");
                            tx_packet_out.send(error_notice.into()).await?;
                            break;
                    },
                }
            }
        }else {
            if bytes_peeked == 0{
                //println!("Read connection to server has closed.");
                ret_string = "Read connection to server has closed.".into();
                break;
            }
        }
    }
    Ok(ret_string.into())
}
