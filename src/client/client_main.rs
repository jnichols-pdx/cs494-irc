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
/*use std::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};*/


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
