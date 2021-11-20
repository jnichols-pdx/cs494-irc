#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::net::{TcpListener, TcpStream};
use std::io::{Write,Read};
use std::collections::HashMap;

use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};

pub struct Client<'a,'b> {
    pub name: String,
    pub connection: TcpStream,
    pub rooms: Vec<&'b Room<'b,'a>>, 
}

pub struct Room<'a,'b> {
    pub name: String,
    pub users: Vec<&'b Client<'b,'a>>,
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Copy,Clone,FromPrimitive)]
enum IrcKind {
    IRC_KIND_ERR = 0x01,
    IRC_KIND_NEW_CLIENT = 0x02,
    IRC_KIND_HEARTBEAT = 0x03,
    IRC_KIND_ENTER_ROOM = 0x04,
    IRC_KIND_LEAVE_ROOM = 0x05,
    IRC_KIND_LIST_ROOMS = 0x06,
    IRC_KIND_ROOM_LISTING = 0x07,
    IRC_KIND_USER_LISTING = 0x08,
    IRC_KIND_QUERY_USER = 0x09,
    IRC_KIND_SEND_MESSAGE = 0x0A,
    IRC_KIND_BROADCAST_MESSAGE = 0x0B,
    IRC_KIND_POST_MESSAGE = 0x0C,
    IRC_KIND_DIRECT_MESSAGE = 0x0D,
    IRC_KIND_OFFER_FILE = 0x0E,
    IRC_KIND_ACCEPT_FILE = 0x0F,
    IRC_KIND_REJECT_FILE = 0x10,
    IRC_KIND_FILE_TRANSFER = 0x11,
    IRC_KIND_CLIENT_DEPARTS = 0x12,
    IRC_KIND_SERVER_DEPARTS = 0x13,
}

pub struct HelloPacket {
    chat_name: [u8; 64],
}

impl<'a> HelloPacket {
    //pub fn as_bytes(self) -> [u8;69] {
    pub fn as_bytes(self) -> BytesMut {
        /*let bytes_out = [0 as u8; 69];
        bytes_out[0] = IrcKind::IRC_KIND_NEW_CLIENT;
        bytes_out[1..5] = (64 as u32).to_be_bytes()[0..4];
        bytes_out[6..69] = self.chat_name;*/

        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
            bytes_out.put_u32(64);
            bytes_out.put_slice(&self.chat_name[..]);
        bytes_out
    }

    pub fn from_bytes(source: &mut Bytes ) -> HelloPacket {
        let kind_raw: IrcKind = FromPrimitive::from_u8(source.get_u8()).unwrap();
        let length: u32 = source.get_u32();
        let name = std::str::from_utf8(source).expect("convertutf8error");
        HelloPacket {
          chat_name: name.as_bytes().try_into().expect("wrongslicelength"),
        }
    }

    pub fn new(name: &'a String) -> Self {
//        match name.find('\x01') {
 //           Some(_) => Err(fmt::Error),
            //None => Ok(HelloPacket {
            let mut buf = [0u8;64];
            let bytesn = name.as_bytes();
            let byteslen = bytesn.len();
            buf[0..byteslen].copy_from_slice(bytesn);
            HelloPacket {
                        chat_name: buf,
            }
  //          })
       // }
    }
}



fn main() -> std::io::Result<()> {
    println!("Hello, world! [server]");
    let listener = TcpListener::bind("0.0.0.0:17734")?;

    
    let mut users = HashMap::<String, Client>::new();
    let mut rooms= HashMap::<String, Room>::new();

    for stream in listener.incoming() {
        let new_client =  handle_client(stream?)?;
        for (_,user) in users.iter_mut() {
            user.connection.write(format!("{} has joined the server!", new_client.name).as_bytes())?;
        }
        users.insert(new_client.name.clone(), new_client);
            
    }
    Ok(())
}

fn handle_client<'a,'b>(mut stream: TcpStream) -> std::io::Result<Client<'a,'b>> {
    stream.set_nodelay(true).expect("Unable to set delay false");
    stream.set_nonblocking(true).expect("Unable to go nonblocking.");
    let mut empty_rooms = Vec::new();


    let mut buffer = [0; 256];
    let mut buff_b = BytesMut::with_capacity(69);
    let mut bytes_read;
    let client_name;
    bytes_read = stream.read(&mut buffer)?;
    if bytes_read> 0 {
       // client_name = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap();
        //println!("{}",std::str::from_utf8(&buffer[0..bytes_read]).unwrap());
       buff_b.extend_from_slice(&buffer[0..69]);
        let pack = HelloPacket::from_bytes(&mut  Bytes::from(buff_b));
        client_name = String::from_utf8(pack.chat_name.to_vec()).unwrap();
    } else {
       client_name = "jane doe".to_string();
    }

    let mut new_client = Client {name: client_name, connection: stream, rooms: empty_rooms};

    println!("customer! '{}' - {}", new_client.name, new_client.name.len());
    new_client.connection.write(format!("welcome {}", new_client.name).as_bytes())?;

    Ok(new_client)
}
