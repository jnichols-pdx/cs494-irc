#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::convert::TryInto;
use thiserror::Error;
use std::io;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Copy,Clone,FromPrimitive)]
pub enum IrcKind {
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

#[derive(Error, Debug)]
pub enum IrcError {

    //IRC Custom errors
    #[error("Cannot Be Empty")]
    InvalidEmpty(),
    #[error("Too Big: {0} bytes used, {1} bytes available")]
    TooManyBytes(i32, i32),
    #[error("Invalid Name Content")]
    InvalidNameContent(),
    #[error("Name Too Long: {0} codepoints")]
    NameTooLong(i32),
    #[error("Invalid Message Content")]
    InvalidMessageContent(),
    #[error("Message Too Long: {0} codepoints")]
    MessageTooLong(i32),
    #[error("Invalid Filename Content")]
    InvalidFilenameContent(),
    #[error("Filename Too Long: {0} codepoints")]
    FilenameTooLong(i32),

    //Wrappers around library errors we may encounter
    #[error("Encountered IO Error: {0}")]
    Io(io::Error),
}

impl From<io::Error> for IrcError {
    fn from(err: io::Error) -> IrcError {
        IrcError::Io(err)
    }
}

/// Result type for directory errors.
pub type Result<'a, T> = std::result::Result<T, IrcError>;



pub struct HelloPacket {
    //pub chat_name: [u8; 64],
    pub chat_name: String,
}


pub fn valid_name<'a>(name: &'a String) -> Result<&'a String> {
        //match name.find
        Ok(name)
    }

impl HelloPacket {
    //pub fn as_bytes(self) -> [u8;69] {
    pub fn as_bytes(self) -> BytesMut {
        /*let bytes_out = [0 as u8; 69];
        bytes_out[0] = IrcKind::IRC_KIND_NEW_CLIENT;
        bytes_out[1..5] = (64 as u32).to_be_bytes()[0..4];
        bytes_out[6..69] = self.chat_name;*/

        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
            bytes_out.put_u32(64);
            bytes_out.put_slice(&self.chat_name.as_bytes());
        bytes_out
    }

    /*pub fn from_Bytes(source: &mut Bytes ) -> HelloPacket {
        let kind_raw: IrcKind = FromPrimitive::from_u8(source.get_u8()).unwrap();
        let length: u32 = source.get_u32();
        let name = String::from_utf8(source.to_vec()).expect("convertutf8error");
        HelloPacket {
          chat_name: name,
        }
    }*/

    pub fn from_bytes(source: &[u8] ) -> HelloPacket {
        let kind_raw: IrcKind = FromPrimitive::from_u8(source[0]).unwrap();
        let length: u32 = 64;//source.get_u32();
        let name = std::str::from_utf8(&source[5..]).expect("convertutf8error");
        HelloPacket {
          chat_name: name.try_into().expect("wrongslicelength"),
        }
    }
    pub fn new(name: & String) -> Result<Self> {
//        match name.find('\x01') {
 //           Some(_) => Err(fmt::Error),
            //None => Ok(HelloPacket {
            //let mut buf = [0u8;64];
            //let bytesn = name.as_bytes();
            //let byteslen = bytesn.len();
            //buf[0..byteslen].copy_from_slice(bytesn);
            let v_name = valid_name(name)?;
            Ok(HelloPacket {
                        chat_name: v_name.clone(),
            })
  //          })
       // }
    }

}

