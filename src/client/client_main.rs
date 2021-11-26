#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::{*};

use std::env;
use tokio::net::TcpStream;
//use std::net::TcpStream;
//use std::io::{Write,Read};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::{self, Duration};
use std::error::Error;
use num_enum::FromPrimitive;
//use bytes::{Bytes, BytesMut, Buf, BufMut};*/

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;


use cursive::Cursive;
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

    let mut arg_list = env::args().skip(1);
    let my_name = arg_list.next().unwrap();
    println!("Hello, world! [client]:{:?}",my_name);

    let host;
    if arg_list.len() > 0 {
        host = arg_list.next().unwrap();
        println!("going to host {}", host);
    } else {
        host = "192.168.2.5:17734".to_string();
        println!("going to default host {}", host);
    }


    let ident = NewClientPacket::new(&my_name)?;

    println!("about to con");
    let mut con = TcpStream::connect(host).await?;
    //con.set_nodelay(true).expect("Unable to set nodelaay");
    println!("conned, about to ident");
    con.write(&ident.as_bytes()).await?;
    println!("idented, waiting for resposne");

    let runner = tokio::spawn(reader(con)); 
    let stopper = tokio::spawn(shutdown_monitor(r2));

    let ui = thread::spawn(move || {
	    let mut siv = cursive::default();
	    siv.run();
    });

    tokio::select!{
        out =  runner => {println!("We stopping with {:?}",out?);},
        _ = stopper => {println!("CTL-c out of the select");},
    }

    Ok(())

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


async fn reader<'a>(mut con: TcpStream) -> Result<'a, ()> {
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
