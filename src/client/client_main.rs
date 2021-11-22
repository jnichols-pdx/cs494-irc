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

    let mut buffer = [0; 256];
    let mut con = TcpStream::connect("192.168.2.5:17734")?;
    con.write(&ident.as_bytes())?;
    let mut bytes_read;
   
    loop {
        println!("------");
        bytes_read = con.read(&mut buffer)?;
        if bytes_read> 0 {
            let kind_raw = IrcKind::from(buffer[0]);
            match  kind_raw {
                IrcKind::IRC_KIND_ERR =>{
                            let my_error = ErrorPacket::from_bytes(&buffer[0..6])?;
                            if my_error.error_code == IrcErrCode::IRC_ERR_NAME_IN_USE {
                                println!("Bogus! that name's taken!");
                            }
                },
                IrcKind::IRC_KIND_HEARTBEAT => {

                     println!("heartbeat!");
                },
                //_ => {println!("unknwon packet");},
                _ => println!("{}",std::str::from_utf8(&buffer[0..bytes_read]).unwrap()),

            }
        }else {
            break;
        }
    }
    
    Ok(())

}
