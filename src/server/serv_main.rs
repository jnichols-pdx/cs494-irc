#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::{*};

use std::net::{TcpListener, TcpStream};
use std::io::{Write,Read};
use std::collections::HashMap;
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
