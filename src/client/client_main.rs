#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//#[macro_use]
//extern crate num_derive;

use std::env;
use std::net::TcpStream;
use std::io::{Write,Read};
use std::convert::TryInto;
use std::fmt;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};

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

fn main() -> std::io::Result<()>{
    let my_name = env::args().skip(1).next().unwrap();
    println!("Hello, world! [client]:{:?}",my_name);

    //let ident = HelloPacket::new(&my_name).expect("bad name");
    let ident = HelloPacket::new(&my_name);

    let mut buffer = [0; 256];
    //let mut con = TcpStream::connect("127.0.0.1:17734")?;
    let mut con = TcpStream::connect("192.168.2.5:17734")?;
    //con.write(my_name.as_bytes())?;
    con.write(&ident.as_bytes())?;
    let mut bytes_read;
   
    loop {
        println!("------");
        bytes_read = con.read(&mut buffer)?;
        if bytes_read> 0 {
            //if readL < 256 { buffer[readL] = 0}
            println!("{}",std::str::from_utf8(&buffer[0..bytes_read]).unwrap());
        } else {
            break;
        }
    }
    
    Ok(())

}
