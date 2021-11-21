#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::convert::TryInto;
use thiserror::Error;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

/// Result type for IRC  errors.
pub type Result<'a, T> = std::result::Result<T, IrcError>;

#[allow(non_camel_case_types)]
//#[allow(dead_code)]
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
    TooManyBytes(usize, usize),
    #[error("Invalid Name Content")]
    InvalidNameContent(),
    #[error("Name Too Long: {0} codepoints")]
    NameTooLong(usize),
    #[error("Invalid Message Content")]
    InvalidMessageContent(),
    #[error("Message Too Long: {0} codepoints")]
    MessageTooLong(usize),
    #[error("Invalid Filename Content")]
    InvalidFilenameContent(),
    #[error("Filename Too Long: {0} codepoints")]
    FilenameTooLong(usize),

    //Wrappers around library errors we may encounter
    #[error("Encountered IO Error: {0}")]
    Io(io::Error),
}

impl From<io::Error> for IrcError {
    fn from(err: io::Error) -> IrcError {
        IrcError::Io(err)
    }
}



pub fn valid_name<'a>(name: &'a String) -> Result<&'a String> {
    //NAMES
    //must be 64 bytes or less in utf-8 encoding,
    //must be 32 codepoints or less,
    //must not have 0x00-0x20 (low ascii command codes),
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)

    let byte_size = name.len();
    if byte_size > 64 {
        return Err(IrcError::TooManyBytes(byte_size, 64));
    }


    let num_points = name.chars().count();
    if num_points  == 0 {
        return Err(IrcError::InvalidEmpty());
    }
    if num_points > 32 {
        return Err(IrcError::NameTooLong(num_points));
    }

    lazy_static! {
        static ref REN: Regex = Regex::new("[\u{00}-\u{1F}\u{20}\u{202A}-\u{202E}\u{2066}-\u{2069}\u{200E}\u{200F}\u{061C}]").unwrap();
    }
    if REN.is_match(name) {
        return Err(IrcError::InvalidNameContent());
    }

    Ok(name)
}




pub struct HelloPacket {
    //pub chat_name: [u8; 64],
    pub chat_name: String,
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


#[test]
fn test_reject_name_chars() {
        assert!(valid_name(&"blah".to_string()).is_ok());  //text is ok

        //ascii low control chars are not
        assert!(valid_name(&"bla    h".to_string()).is_err()); //ascii tab is in low control, not ok
        assert!(valid_name(&"bla\x00h".to_string()).is_err());
        assert!(valid_name(&"bla\x01h".to_string()).is_err());
        assert!(valid_name(&"bla\x02h".to_string()).is_err());
        assert!(valid_name(&"bla\x03h".to_string()).is_err());
        assert!(valid_name(&"bla\x04h".to_string()).is_err());
        assert!(valid_name(&"bla\x05h".to_string()).is_err());
        assert!(valid_name(&"bla\x06h".to_string()).is_err());
        assert!(valid_name(&"bla\x07h".to_string()).is_err());
        assert!(valid_name(&"bla\x08h".to_string()).is_err());
        assert!(valid_name(&"bla\x09h".to_string()).is_err());
        assert!(valid_name(&"bla\x0Ah".to_string()).is_err());
        assert!(valid_name(&"bla\x0Bh".to_string()).is_err());
        assert!(valid_name(&"bla\x0Ch".to_string()).is_err());
        assert!(valid_name(&"bla\x0Dh".to_string()).is_err());
        assert!(valid_name(&"bla\x0Eh".to_string()).is_err());
        assert!(valid_name(&"bla\x0Fh".to_string()).is_err());
        assert!(valid_name(&"bla\x10h".to_string()).is_err());
        assert!(valid_name(&"bla\x11h".to_string()).is_err());
        assert!(valid_name(&"bla\x12h".to_string()).is_err());
        assert!(valid_name(&"bla\x13h".to_string()).is_err());
        assert!(valid_name(&"bla\x14h".to_string()).is_err());
        assert!(valid_name(&"bla\x15h".to_string()).is_err());
        assert!(valid_name(&"bla\x16h".to_string()).is_err());
        assert!(valid_name(&"bla\x17h".to_string()).is_err());
        assert!(valid_name(&"bla\x18h".to_string()).is_err());
        assert!(valid_name(&"bla\x19h".to_string()).is_err());
        assert!(valid_name(&"bla\x1Ah".to_string()).is_err());
        assert!(valid_name(&"bla\x1Bh".to_string()).is_err());
        assert!(valid_name(&"bla\x1Ch".to_string()).is_err());
        assert!(valid_name(&"bla\x1Dh".to_string()).is_err());
        assert!(valid_name(&"bla\x1Eh".to_string()).is_err());
        assert!(valid_name(&"bla\x1Fh".to_string()).is_err());


        //spaces are not ok
        assert!(valid_name(&"bla h".to_string()).is_err());
        assert!(valid_name(&"bla\x20h".to_string()).is_err());

        //bidi go byebye
        assert!(valid_name(&"bla\u{061C}h".to_string()).is_err());

        assert!(valid_name(&"bla\u{200E}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{200F}h".to_string()).is_err());

        assert!(valid_name(&"bla\u{202A}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{202B}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{202C}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{202D}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{202E}h".to_string()).is_err());

        assert!(valid_name(&"bla\u{2066}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{2067}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{2068}h".to_string()).is_err());
        assert!(valid_name(&"bla\u{2069}h".to_string()).is_err());
}

#[test]
fn test_reject_name_length() {
        assert!(valid_name(&"hunter2".to_string()).is_ok());  //short names are OK

        assert!(valid_name(&"abcdefghijklmnopqrstuvwxyz1234567890".to_string()).is_err()); //long names are not

        assert!(valid_name(&"12345678901234567890123456789012".to_string()).is_ok()); //max length is OK
        assert!(valid_name(&"123456789012345678901234567890123".to_string()).is_err()); //one more is not
        assert!(valid_name(&"".to_string()).is_err()); //empty strings are not
        assert!(valid_name(&"123456789012345678901234567890™™".to_string()).is_ok()); //multibyte unicode count as one and are OK
        assert!(valid_name(&"123456789012345678901234567890™™™".to_string()).is_err()); //multibyte unicode count as one, but may still push us over the limit.

        assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™".to_string()).is_ok()); //63 bytes is OK
        assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™A".to_string()).is_ok()); //64 bytes is OK
        assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™AB".to_string()).is_err()); //65 bytes is not OK
        assert!(valid_name(&"™™™™™™™™™™™™™™™™™™™™™™".to_string()).is_err()); //66 bytes is not ok


}
