#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//#[macro_use]
//extern crate num_derive;
//

use irclib::{*};

use std::env;
use std::net::TcpStream;
use std::io::{Write,Read};
use std::error::Error;
use num_enum::FromPrimitive;
//use bytes::{Bytes, BytesMut, Buf, BufMut};*/
//use irclib::IrcPacket;

fn main() -> Result<'static, ()>{
    let my_name = env::args().skip(1).next().unwrap();
    println!("Hello, world! [client]:{:?}",my_name);

    let ident = NewClientPacket::new(&my_name)?;

    let mut peeker = [0; 5];
    let mut con = TcpStream::connect("192.168.2.5:17734")?;
    con.set_nodelay(true).expect("Unable to set nodelaay");
    con.write(&ident.as_bytes())?;
    let mut bytes_peeked;

    loop {
        bytes_peeked = con.peek(&mut peeker)?;
        if bytes_peeked == 5 {
            println!("------");
            println!("{}.{}.{}.{}.{}", peeker[0],peeker[1],peeker[2],peeker[3],peeker[4]);
            let kindbyte = peeker[0];
            let msg_len = u32_from_slice(&peeker[1..5]) as usize;
            let mut buffer = vec![0; msg_len + 5];
            let bytes_read = con.read(&mut buffer)?;
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
            break;
        }
    }


    Ok(())

}
