#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use irclib::{*};

use tokio::net::{TcpListener, TcpStream};
//use std::net::{TcpListener, TcpStream, Shutdown};
//use std::net::{Shutdown};
//use std::io::{Write,Read};
use std::io::{ErrorKind};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use bytes::{Bytes, BytesMut, Buf, BufMut};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;



pub struct Client<'a,'b> {
    pub name: String,
    pub connection: TcpStream,
    pub rooms: Vec<&'b Room<'b,'a>>, 
}

pub struct Room<'a,'b> {
    pub name: String,
    pub users: Vec<&'b Client<'b,'a>>,
}



#[tokio::main]
async fn main() -> Result<'static, ()> {
    println!("Hello, world! [server]");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");


    let listener = TcpListener::bind("0.0.0.0:17734").await?;

        let heart = HeartbeatPacket::new()?;
    
    let mut users = HashMap::<String, Client>::new();
    let mut rooms= HashMap::<String, Room>::new();


//    for stream in listener.incoming() {

    loop {
        let (socket, _) = listener.accept().await?;

        if !running.load(Ordering::SeqCst) {
            //we've been asked to close - so send some cleanup packets!
            println!("ctrl-c shutdown");

            let outgoing = ServerDepartsPacket::new(&"Taking a nap, seeya!".to_string())?;
            for (_,user) in users.iter_mut() {
                user.connection.write(&outgoing.as_bytes()).await?;
                //user.connection.shutdown(Shutdown::Both).expect("blah - couldn't say bye");
                user.connection.shutdown().await?;
            }
            return Ok(());
        }
        let mut new_client =  handle_client(socket)?;
        if users.contains_key(&new_client.name) {
            println!("dup shutdown");
            new_client.connection.write(&ErrorPacket::new(IrcErrCode::IRC_ERR_NAME_IN_USE)?.as_bytes()).await?;
            //new_client.connection.shutdown(Shutdown::Both)?;
            new_client.connection.shutdown().await?;
        }else {
            let newbie_name = new_client.name.clone();
            {
            spam_user(&mut new_client).await?;
            }
            users.insert(new_client.name.clone(), new_client);
            let outgoing = PostMessagePacket::new(&"all".to_string(),&newbie_name, &format!("{} has joined the server!", &newbie_name))?;
            for (_,user) in users.iter_mut() {
                user.connection.write(&outgoing.as_bytes()).await?;
                user.connection.write(&heart.as_bytes()).await?;
            }
        }
            
    }
//    Ok(())
}

// fn handle_client<'a,'b>(mut stream: TcpStream) -> std::io::Result<Client<'a,'b>> {
fn handle_client<'a,'b>(mut stream: TcpStream) -> std::io::Result<Client<'a,'b>> {
    stream.set_nodelay(true).expect("Unable to set delay false");
    //stream.set_nonblocking(true).expect("Unable to go nonblocking.");
    let mut empty_rooms = Vec::new();


    let mut buffer = [0u8; 256];
    let mut buff_b = BytesMut::with_capacity(69);
    let mut bytes_read : usize = 0;
    let client_name;

    loop {
        let result = stream.try_read(&mut buffer);
        match result {
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                break;
            }
            Ok(0) => break,
            Ok(n) => {bytes_read = n;break},
        };
    }

    /*match bytes_read {
        Ok(0) => return Err(IrcError::PacketMismatch()), //placeholder*/
        
    if bytes_read> 0 {
       // client_name = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap();
        //println!("{}",std::str::from_utf8(&buffer[0..bytes_read]).unwrap());
        buff_b.extend_from_slice(&buffer[0..69]);
        let pack = NewClientPacket::from_bytes(&mut  Bytes::from(buff_b)).expect("yup");
        //client_name = String::from_utf8(pack.chat_name.to_vec()).unwrap();
        client_name = pack.chat_name;
    } else {
       client_name = "jane doe".to_string();
    }

    let mut new_client = Client {name: client_name, connection: stream, rooms: empty_rooms};

    println!("customer! '{}' - {}", new_client.name, new_client.name.len());
    //new_client.connection.write(format!("welcome {}", new_client.name).as_bytes())?;

    Ok(new_client)
}

async fn spam_user<'a,'b,'c>(dude: &mut Client<'b,'c>) -> Result<'a, ()> {
        println!("I'm at spammmm!");


                    let mut ncp = NewClientPacket::new(&"Jeff".into())?;
                    let mut err1 = ErrorPacket::new(IrcErrCode::IRC_ERR_UNKNOWN)?;
                    let mut err2 = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_KIND)?;
                    let mut err3 = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_LENGTH)?;
                    let mut err4 = ErrorPacket::new(IrcErrCode::IRC_ERR_NAME_IN_USE)?;
                    let mut err5 = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_NAME)?;
                    let mut err6 = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_MESSAGE)?;
                    let mut err7 = ErrorPacket::new(IrcErrCode::IRC_ERR_ILLEGAL_TRANSFER)?;
                    let mut err8 = ErrorPacket::new(IrcErrCode::IRC_ERR_TOO_MANY_USERS)?;
                    let mut err9 = ErrorPacket::new(IrcErrCode::IRC_ERR_TOO_MANY_ROOMS)?;

//                    IrcKind::IRC_KIND_HEARTBEAT => {

                    let mut erp = EnterRoomPacket::new(&"r/Politics".into())?;
                    let mut lrp = LeaveRoomPacket::new(&"r/WorldPeace".into())?;
                    let mut lip = ListRoomsPacket::new()?;
                    let mut rlp = RoomListingPacket::new()?;
                        rlp.push(&"Alpha".into())?;
                        rlp.push(&"Beta".into())?;
                        rlp.push(&"Gamma".into())?;
                    let mut ulp = UserListingPacket::new()?;
                        ulp.push(&"Ada".into())?;
                        ulp.push(&"Ben".into())?;
                        ulp.push(&"Charlie".into())?;
                    let mut qup = QueryUserPacket::new(&"Pedro".into())?;
                        qup.set_online();
                    let mut smp = SendMessagePacket::new(&"r/Politics".into(), &"Things happened today.".into())?;
                    let mut bmp = BroadcastMessagePacket::new(&"Server announcement: today is taco tuesday!".into())?;

                   // IrcKind::IRC_KIND_POST_MESSAGE => {
                   
                    let mut dmp = DirectMessagePacket::new(&"AuntMable".into(), &"So I says to Kathleen, I says to her...".into())?;
                    let mut ofp = OfferFilePacket::new(&"Your_Sister".into(), &"Your_Mother".into(), 3524, &"Recipe.txt".into())?;
                    let mut afp = AcceptFilePacket::new(&"Your_Sister".into(), &"Your_Mother".into(), 15, 3524, &"Recipe.txt".into())?;
                    let mut rfp = RejectFilePacket::new(&"Your_Sister".into(), &"Your_Mother".into(), 15,  3524, &"Recipe.txt".into())?;
                    let mut ftp = FileTransferPacket::new(15, false, Bytes::from_static(b"Mix dry ingredients in small bowl..."))?;
                    let mut cdp = ClientDepartsPacket::new(&"Arrivaderci!".into())?;
                    //IrcKind::IRC_KIND_SERVER_DEPARTS => {


                    dude.connection.write(&ncp.as_bytes()).await?;
                    /*dude.connection.write(&err1.as_bytes()).await?;
                    dude.connection.write(&err2.as_bytes()).await?;
                    dude.connection.write(&err3.as_bytes()).await?;
                    dude.connection.write(&err4.as_bytes()).await?;
                    dude.connection.write(&err5.as_bytes()).await?;
                    dude.connection.write(&err6.as_bytes()).await?;
                    dude.connection.write(&err7.as_bytes()).await?;
                    dude.connection.write(&err8.as_bytes()).await?;
                    dude.connection.write(&err9.as_bytes()).await?;*/
                    dude.connection.write(&erp.as_bytes()).await?;
                    dude.connection.write(&lrp.as_bytes()).await?;
                    dude.connection.write(&lip.as_bytes()).await?;
                    dude.connection.write(&rlp.as_bytes()).await?;
                    dude.connection.write(&ulp.as_bytes()).await?;
                    dude.connection.write(&qup.as_bytes()).await?;
                    dude.connection.write(&smp.as_bytes()).await?;
                    dude.connection.write(&bmp.as_bytes()).await?;
                    dude.connection.write(&dmp.as_bytes()).await?;
                    dude.connection.write(&ofp.as_bytes()).await?;
                    dude.connection.write(&afp.as_bytes()).await?;
                    dude.connection.write(&rfp.as_bytes()).await?;
                    dude.connection.write(&ftp.as_bytes()).await?;
                    dude.connection.write(&cdp.as_bytes()).await?;
                    dude.connection.write(b"\x54\0\0\0\x02\x01\x03").await?;

                Ok(())
}
