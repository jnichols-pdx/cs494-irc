#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::{*};

use std::env;
use std::error::Error;
use num_enum::FromPrimitive;

use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::{self, Duration};
use tokio::sync::{mpsc, oneshot};
use bytes::{Bytes, BytesMut, Buf, BufMut};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;


use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
use std::thread;


#[tokio::main]
async fn main() -> Result<'static, ()>{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || { 
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    let r2 = running.clone();
    let r3 = running.clone();


    let mut arg_list = env::args().skip(1);
    let my_name = arg_list.next().unwrap();
    println!("Hello, world! [client]:{:?}",my_name);

    let host;
    if arg_list.len() > 0 {
        host = arg_list.next().unwrap();
        println!("Connecting to host {}", host);
    } else {
        host = "192.168.2.5:17734".to_string();
        println!("Connecting to default host {}", host);
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

    //Oneshot to retrieve a sender to the Cursive UI's callback channel from the kickstart
    //function.
    let (tx_cb_to_main, mut rx_cb_from_kickstart) = oneshot::channel::<cursive::CbSink>();
    
    //Start the console UI in it's own OS thread (doesn't play nice with tokio's
    //task management green threads).
    let ui_thread = thread::spawn(move || {ui_kickstart(r3, tx_cb_to_main, tx2)});

    //Retrieve a handle to the UI's callback endpoint, so we may tell it to react to incoming
    //packets.
    let cb_handle;
    match rx_cb_from_kickstart.await {
        Ok(v) => {cb_handle= v},
        Err(e) => {println!("Got Err instead of handle{:?}",e);return Err(IrcError::PacketMismatch());},
    }

    //Split the TcpStream into reader and writer, pass each to their own asynchronous task
    let (tcp_in, tcp_out) = con.into_split();
    //Spawn asynchronous tokio tasks to watch for shutdown triggers, receive messages from, and
    //send messages to the IRC server.
    let stop_task = tokio::spawn(shutdown_monitor(r2));
    let read_task = tokio::spawn(reader(tcp_in, tx_to_responder)); 
    let responder_task = tokio::spawn(responder(cb_handle,ui_rx)); 
    let send_task = tokio::spawn(writer(tcp_out ,outgoing_rx));
    let heartbeat_task = tokio::spawn(pulse(tx3));

    //Polls the tokio tasks to keep them operating, ends all tasks when one completes
    //stop_task will complete when the Cursive UI event loop exits (explicit quit, or catches ctrl-c)
    //    or if our direct ctrl-c handler gets a ctrl-c signal that evaded Cursive's runtime.
    //read_task will complete when the incoming network connection from the server closes, or we 
    //    recieve a ServerDepart message indicating the server will close.
    //responder_task may complete when read_task drops the communication channel they share
    //send_task will complete with the outgoing network connection to the server closes.
    tokio::select!{
        out = read_task => {println!("We stopping with {:?}",out?);},
        _ = responder_task => {println!("responder died:(");},
        _ = stop_task => {println!("CTL-c out of the select");},
        _ = send_task => {println!("Outgoing channel died...");},
    }

    Ok(())

}

fn ui_kickstart(running: Arc<AtomicBool>, tx_return_cb_handle: tokio::sync::oneshot::Sender<cursive::CbSink>, tx_packet_out: tokio::sync::mpsc::Sender<irclib::SyncSendPack>)
{
	let mut siv = cursive::default();
    let cb = siv.cb_sink().clone();
    tx_return_cb_handle.send(cb).expect("Couldn't pass back cb handle");


    siv.run();

    running.store(false, Ordering::SeqCst);

}

async fn pulse<'a>(tx_packet_out: tokio::sync::mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()>
{
    let mut wait_period = time::interval(Duration::from_millis(5000));
    loop {
        wait_period.tick().await;
        let heartbeat = HeartbeatPacket::new().expect("Heartbeat packets should be infallible on creation");
        tx_packet_out.send(heartbeat.into()).await?;
    }
}

async fn responder(cb: cursive::CbSink,mut rx_from_main: mpsc::Receiver<SyncSendPack>)
{

    while let Some(packet) = rx_from_main.recv().await {
        println!("parse me packets!");
    }
}


async fn shutdown_monitor(running: Arc<AtomicBool>)
{
    let mut wait_period = time::interval(Duration::from_millis(100));
    loop {

        wait_period.tick().await;
        if !running.load(Ordering::SeqCst) {
            //we've been asked to close - so send some cleanup packets!
            println!("ctrl-c shutdown");

            //TODO: communication accross threads for sending...
            //let outgoing = ClientDepartsPacket::new(&"Client going to vegas".to_string())?;
            //    con.write(&outgoing.as_bytes()).await?;
            
            break;
        }

        //TODO: also check for communication from Cursive :3
    }

}

async fn writer<'a>(mut con: tokio::net::tcp::OwnedWriteHalf, mut rx_packets_to_send: mpsc::Receiver<SyncSendPack>) -> Result<'a,()> {
    while let Some(packet) = rx_packets_to_send.recv().await {
        println!("send me packets!");
    }
    Ok(())

}

async fn reader<'a>(mut con: tokio::net::tcp::OwnedReadHalf, tx_to_responder: mpsc::Sender<SyncSendPack>) -> Result<'a, ()> {
    println!("in fn");
    let mut peeker = [0; 5];
    let mut bytes_peeked;
    loop {
    println!("in loop");
        bytes_peeked = con.peek(&mut peeker).await?;
        if bytes_peeked == 5 {
            println!("------");
            println!("{}.{}.{}.{}.{}", peeker[0],peeker[1],peeker[2],peeker[3],peeker[4]);
            let kindbyte = peeker[0];
            let msg_len = u32_from_slice(&peeker[1..5]) as usize;
            let mut buffer = vec![0; msg_len + 5];
            let bytes_read = con.read(&mut buffer).await?;
            println!("got {} bytes, expected {}", bytes_read, msg_len +5);
            println!("{:?}", buffer);
            if bytes_read == msg_len + 5 {
                let kind_raw = IrcKind::from(buffer[0]);
                match  kind_raw {
                    IrcKind::IRC_KIND_NEW_CLIENT => { println!("Got New client packet...?");},
                    IrcKind::IRC_KIND_ERR => {
                        let my_error = ErrorPacket::from_bytes(&buffer[0..6])?;

                        match my_error.error_code {
                            IrcErrCode::IRC_ERR_UNKNOWN => { println!("Bogus! Server's confused (err Unknown)");},
                            IrcErrCode::IRC_ERR_ILLEGAL_KIND => { println!("Bogus! Illegal Kind!");},
                            IrcErrCode::IRC_ERR_ILLEGAL_LENGTH => { println!("Bogus! Illegal Length!");},
                            IrcErrCode::IRC_ERR_NAME_IN_USE => { println!("Bogus! That name's taken!");},
                            IrcErrCode::IRC_ERR_ILLEGAL_NAME => { println!("Bogus! Illegal Name!");},
                            IrcErrCode::IRC_ERR_ILLEGAL_MESSAGE => { println!("Bogus! Illegal Message!");},
                            IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER => { println!("Bogus! Illegal Transfer!");},
                            IrcErrCode::IRC_ERR_TOO_MANY_USERS => { println!("Bogus! Slashdoted! (too many users)");},
                            IrcErrCode::IRC_ERR_TOO_MANY_ROOMS => { println!("Bogus! Too Many Rooms!");},
                            _ => (),
                        }
                    },
                    IrcKind::IRC_KIND_HEARTBEAT => {

                        println!("heartbeat!");
                    },
                    IrcKind::IRC_KIND_ENTER_ROOM => {println!("Got enter room packet...?");},
                    IrcKind::IRC_KIND_LEAVE_ROOM => {println!("Got leave room packet...?");},
                    IrcKind::IRC_KIND_LIST_ROOMS => {println!("Got list rooms packet...?");},
                    IrcKind::IRC_KIND_ROOM_LISTING => {
                        println!("Got room listing packet.");
                        let room_list = RoomListingPacket::from_bytes(&buffer[..])?;
                        for room in room_list.rooms {
                            println!("-{}",room);
                        };
                    },
                    IrcKind::IRC_KIND_USER_LISTING => {
                        println!("Got user listing packet.");
                        let user_list = UserListingPacket::from_bytes(&buffer[..])?;
                        for user in user_list.users{
                            println!("-{}", user);
                        };
                    },
                    IrcKind::IRC_KIND_QUERY_USER => {
                        println!("Got query user packet.");
                        let query_result = QueryUserPacket::from_bytes(&buffer[..])?;
                        println!("{} is {}", &query_result.user_name, &query_result.status);
                    },
                    IrcKind::IRC_KIND_SEND_MESSAGE => {println!("Got send message packet...?");},
                    IrcKind::IRC_KIND_BROADCAST_MESSAGE => {println!("Got broadcast message packet...?");},
                    IrcKind::IRC_KIND_POST_MESSAGE => {
                        let new_message = PostMessagePacket::from_bytes(&buffer[..])?;
                        println!("{}: {}", &new_message.sender, &new_message.message);

                    },



                    IrcKind::IRC_KIND_DIRECT_MESSAGE => {
                        let new_direct = DirectMessagePacket::from_bytes(&buffer[..])?;
                        println!("DM from {}: {}", &new_direct.target, &new_direct.message);
                    },
                    IrcKind::IRC_KIND_OFFER_FILE => {println!("Got offer file packet.");},
                    IrcKind::IRC_KIND_ACCEPT_FILE => {println!("Got accept file packet.");},
                    IrcKind::IRC_KIND_REJECT_FILE => {println!("Got reject file packet.");},
                    IrcKind::IRC_KIND_FILE_TRANSFER => {println!("Got file transfer packet.");},
                    IrcKind::IRC_KIND_CLIENT_DEPARTS => {println!("Got client departs packet...?");},
                    IrcKind::IRC_KIND_SERVER_DEPARTS => {
                        println!("Got server departs packet.");
                        let  server_leaving = ServerDepartsPacket::from_bytes(&buffer[..])?;
                        println!("Goodbye: {}", server_leaving.get_message());
                        },
                    _ => println!("Unknown packet:\n{:?}",&buffer[0..bytes_read]),

                }
            }
        }else {
            println!("aw shit");
            break;
        }
    }
    Ok(())

}
