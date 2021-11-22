#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use num_derive::FromPrimitive;
//use num_traits::FromPrimitive;
use num_enum::FromPrimitive;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::convert::{TryInto, TryFrom};
use thiserror::Error;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

/// Result type for IRC  errors.
pub type Result<'a, T> = std::result::Result<T, IrcError>;

#[allow(non_camel_case_types)]
//#[allow(dead_code)]
#[derive(Copy,Clone,FromPrimitive,PartialEq)]
#[repr(u8)]
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

    #[num_enum(default)]
    NO_MATCH_IRC_KIND,
}

#[allow(non_camel_case_types)]
//#[allow(dead_code)]
#[derive(Copy,Clone,FromPrimitive,PartialEq)]
#[repr(u8)]
pub enum IrcErrCode {
    IRC_ERR_UNKNOWN = 0x01,
    IRC_ERR_ILLEGAL_KIND = 0x02,
    IRC_ERR_ILLEGAL_LENGTH = 0x03,
    IRC_ERR_NAME_IN_USE = 0x04,
    IRC_ERR_ILLEGAL_NAME = 0x05,
    IRC_ERR_ILLEGAL_MESSAGE = 0x06,
    IRC_ERR_ILLEGAL_TRANSFER = 0x07,
    IRC_ERR_TOO_MANY_USERS = 0x08,
    IRC_ERR_TOO_MANY_ROOMS = 0x09,

    #[num_enum(default)]
    NO_MATCH_IRC_ERR,
}

//Internal Rust errors, NOT necessarily equivalent to the IRC_ERR_* values contained in an
//irc_packet_error message sent between client and server (aka not ErrorPackets).
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

    #[error("Packet size invalid, found {0} expected {1}")]
    PacketLengthIncorrect(usize, usize),

    #[error("Field size invalid")]
    FieldLengthIncorrect(),

    #[error("Packet kind invalid")]
    PacketMismatch(),

    /*#[error("This packet was malformed")]
    MalformedPacket(),*/

    //Wrappers around library errors we may encounter
    #[error("Encountered IO Error: {0}")]
    Io(io::Error),

    #[error("Encountered FromUTF8 Error: {0}")]
    FromUtf8Err(std::string::FromUtf8Error),

    #[error("Encountered UTF8 Error: {0}")]
    Utf8Err(std::str::Utf8Error),

    #[error("Code out of range")]
    CodeOutOfRange(),
}

impl From<io::Error> for IrcError {
    fn from(err: io::Error) -> IrcError {
        IrcError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for IrcError {
    fn from(err: std::string::FromUtf8Error) -> IrcError {
        IrcError::FromUtf8Err(err)
    }
}

impl From<std::str::Utf8Error> for IrcError {
    fn from(err: std::str::Utf8Error) -> IrcError {
        IrcError::Utf8Err(err)
    }
}


///////////////////////////////////////////////
// UTIL funcs
///////////////////////////////////////////////


pub fn valid_name<'a>(name: &'a String) -> Result<&'a String> {
    //NAMES
    //must be 64 bytes or less in utf-8 encoding,
    //must be more than 0 and less than 32 codepoints,
    //must not have 0x00-0x1f (low ascii command codes),
    //must not have 0x20 (space character)
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

pub fn valid_message<'a>(message: &'a String) -> Result<&'a String> {
    //MESSAGES
    //must be 12000 bytes or less in utf-8 encoding,
    //must be more than 0 codepoints,
    //must not have 0x00-0x19 (low ascii command codes),
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)
    //must have \00 as the final byte.

    let byte_size = message.len();
    if byte_size > 12000 {
        return Err(IrcError::TooManyBytes(byte_size, 64));
    }

    let num_points = message.chars().count();
    if num_points  == 0 {
        return Err(IrcError::InvalidEmpty());
    }

    lazy_static! {
        static ref REM: Regex = Regex::new("[\u{01}-\u{08}\u{0A}-\u{1F}\u{202A}-\u{202E}\u{2066}-\u{2069}\u{200E}\u{200F}\u{061C}]").unwrap();
    }
    if REM.is_match(message) {
        return Err(IrcError::InvalidMessageContent());
    }

    match message.find('\x00') {
        None => {return Err(IrcError::InvalidMessageContent());},  //must end with one
        Some(pos) => {
            if pos != byte_size -1 { //may not appear before the end
                return Err(IrcError::InvalidMessageContent());
            }
        }
    };

    Ok(message)
}

pub fn valid_filename<'a>(file_name: &'a String) -> Result<&'a String> {
    //FILE NAMES
    //may be up to 1024 bytes in utf-8 encoding,
    //must be more than 0 codepoints,
    //must not have 0x00-0x19 (low ascii command codes),
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)
    //must not have 0x003A or 0x002F (file system path delimiters ':' and '/')

    let byte_size = file_name.len();
    if byte_size > 1024 {
        return Err(IrcError::TooManyBytes(byte_size, 1024));
    }


    let num_points = file_name.chars().count();
    if num_points  == 0 {
        return Err(IrcError::InvalidEmpty());
    }

    lazy_static! {
        static ref REFN: Regex = Regex::new("[\u{00}-\u{1F}\u{202A}-\u{202E}\u{2066}-\u{2069}\u{200E}\u{200F}\u{061C}\u{003A}\u{002F}]").unwrap();
    }
    if REFN.is_match(file_name) {
        return Err(IrcError::InvalidNameContent());
    }

    if file_name.starts_with(' ') || file_name.ends_with(' ') {
        return Err(IrcError::InvalidNameContent());
    }

    Ok(file_name)
}


pub fn get_four_bytes_as_array(source: &[u8]) -> [u8;4] {
   source.try_into().expect("Slice with incorrect length.")
}

pub fn u32_from_slice(source: &[u8]) -> u32 {
    u32::from_be_bytes(get_four_bytes_as_array(&source[0..4]))
}

pub fn get_sixtyfour_bytes_as_array(source: &[u8]) -> [u8;64] {
   source.try_into().expect("Slice with incorrect length.")
}

pub fn name_from_slice(source: &[u8]) -> Result<String> {
    if source.len() != 64 {
        return Err(IrcError::PacketLengthIncorrect(source.len(), 64));
    }
    match String::from_utf8(get_sixtyfour_bytes_as_array(&source[..]).to_vec()) {
        Ok(mut n) => {
            match n.find('\0') {
                Some(pos) => { n.truncate(pos);
                               Ok(n)
                },
                None => Ok(n),
            }
        },
        Err(e) => Err(IrcError::FromUtf8Err(e))
    }
}

pub fn string_from_slice(source: &[u8]) -> Result<String> {
        let string_str = std::str::from_utf8(source)?;
        let new_string: String  = string_str.into();
        Ok(new_string)
        //let new_name: String = name.try_into();//.expect("wrongslicelength");
        //let new_name: String::from_utf8(&source[5..].try_into());
}

pub trait IrcPacket {
    fn as_bytes(&self) -> BytesMut;

    fn from_bytes(source: &[u8] ) -> Result<Self> where Self: Sized;

}

///////////////////////////////////////////////
//  Error Packet
///////////////////////////////////////////////

pub struct ErrorPacket {
    pub error_code: IrcErrCode,
}

impl ErrorPacket {

    pub fn new(code: IrcErrCode ) -> Result<'static, Self> {
            Ok(ErrorPacket {
                        error_code: code.to_owned(),
            })
    }

}

impl IrcPacket for ErrorPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_ERR as u8);
        bytes_out.put_u32(1);
        bytes_out.put_u8(self.error_code as u8);
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 6 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 6));
        }

        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ERR{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 1 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_error_code: IrcErrCode = IrcErrCode::from(source[5]);
        match new_error_code {
            IrcErrCode::NO_MATCH_IRC_ERR => Err(IrcError::CodeOutOfRange()),
            code => Ok(ErrorPacket { error_code: code, }),
        }
    }
}

///////////////////////////////////////////////
// NewClient Packet
///////////////////////////////////////////////

pub struct NewClientPacket {
    pub chat_name: String,
}

impl NewClientPacket {

    pub fn new(name: & String) -> Result<Self> {
            let v_name = valid_name(name)?;
            Ok(NewClientPacket {
                        chat_name: v_name.to_owned(),
            })
    }

}

impl IrcPacket for NewClientPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_NEW_CLIENT as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.chat_name.as_bytes());
        let remain = 64 - self.chat_name.len(); 
        for x in 1..remain+1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_NEW_CLIENT{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_name =  valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(NewClientPacket {
          chat_name: new_name,
        })
    }

}


///////////////////////////////////////////////
// Heartbeat Packet
///////////////////////////////////////////////

pub struct HeartbeatPacket {
}

impl HeartbeatPacket {

    pub fn new() -> Result<'static, Self> {
            Ok(HeartbeatPacket {})
    }

}

impl IrcPacket for HeartbeatPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(5);
        bytes_out.put_u8( IrcKind::IRC_KIND_HEARTBEAT as u8);
        bytes_out.put_u32(0);
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 5));
        }

        //let kind_raw: IrcKind = FromPrimitive::from_u8(source[0]).unwrap();
        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_HEARTBEAT{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        Ok(HeartbeatPacket {})
    }

}

///////////////////////////////////////////////
// Enter Room Packet
///////////////////////////////////////////////

pub struct EnterRoomPacket {
    pub room_name: String,
}

impl EnterRoomPacket {

    pub fn new(roomname: & String) -> Result<Self> {
            let v_roomname = valid_name(roomname)?;
            Ok(EnterRoomPacket {
                        room_name: v_roomname.to_owned(),
            })
    }

}

impl IrcPacket for EnterRoomPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_ENTER_ROOM as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.room_name.as_bytes());
        let remain = 64 - self.room_name.len(); 
        for x in 1..remain+1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ENTER_ROOM{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_roomname =  valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(EnterRoomPacket {
          room_name: new_roomname,
        })
    }
}

///////////////////////////////////////////////
// Leave Room Packet
///////////////////////////////////////////////

pub struct LeaveRoomPacket {
    pub room_name: String,
}

impl LeaveRoomPacket {

    pub fn new(roomname: & String) -> Result<Self> {
            let v_roomname = valid_name(roomname)?;
            Ok(LeaveRoomPacket {
                        room_name: v_roomname.to_owned(),
            })
    }

}

impl IrcPacket for LeaveRoomPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8( IrcKind::IRC_KIND_LEAVE_ROOM as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.room_name.as_bytes());
        let remain = 64 - self.room_name.len(); 
        for x in 1..remain+1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_LEAVE_ROOM{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_roomname =  valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(LeaveRoomPacket {
          room_name: new_roomname,
        })
    }
}

///////////////////////////////////////////////
// List Rooms Packet
///////////////////////////////////////////////

pub struct ListRoomsPacket {
}

impl ListRoomsPacket {

    pub fn new() -> Result<'static, Self> {
            Ok(ListRoomsPacket {})
    }

}

impl IrcPacket for ListRoomsPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(5);
        bytes_out.put_u8( IrcKind::IRC_KIND_LIST_ROOMS as u8);
        bytes_out.put_u32(0);
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 5));
        }

        //let kind_raw: IrcKind = FromPrimitive::from_u8(source[0]).unwrap();
        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_LIST_ROOMS{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        Ok(ListRoomsPacket {})
    }

}

///////////////////////////////////////////////
// Room Listing Packet
///////////////////////////////////////////////

pub struct RoomListingPacket{
    pub rooms: Vec<String>,
}

impl RoomListingPacket {

    pub fn new() -> Result<'static, Self> {
            Ok(RoomListingPacket {
                rooms: Vec::new()
                })
    }

    pub fn from_vec(new_rooms: &Vec<String>) -> Result<'static, Self> {
            Ok(RoomListingPacket {
                rooms: new_rooms.to_owned() //takes ownership..?
                })
    }

    pub fn push(&mut self, room: &String) -> Result<()> {
        let a_room = valid_name(&room)?.to_owned();
        self.rooms.push(a_room);
        Ok(())
    }


}

impl IrcPacket for RoomListingPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(133);
        bytes_out.put_u8( IrcKind::IRC_KIND_ROOM_LISTING as u8);
        bytes_out.put_u32(64 + (64 * self.rooms.len()) as u32);
        bytes_out.put_bytes(b'\0', 64);
        for room in &self.rooms {
            bytes_out.put_slice(&room.as_bytes());
            let remain = 64 - room.len(); 
            bytes_out.put_bytes(b'\0', remain);
            /*for x in 1..remain+1 {
                bytes_out.put_u8(b'\0');
            }*/
        }
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ROOM_LISTING {
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);

        if length < 128 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let count_rooms: usize = ((length / 64)-1) as usize;

        if source.len() as usize != (count_rooms*64) + 5 + 64{
            return Err(IrcError::PacketLengthIncorrect(source.len(), (count_rooms*64) +5 + 64));
        }

        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        //Ignores bytes 5..69 which are the unused identifier field.
        let mut new_rooms: Vec<String> = Vec::new();

        for offset in 0..count_rooms {
            let new_roomname =  valid_name(&name_from_slice(&source[(offset*64)+5+64..((offset+1)*64)+5+64])?)?.to_owned();
            new_rooms.push(new_roomname);
        }

        Ok(RoomListingPacket {
          rooms: new_rooms,
        })
    }
}

///////////////////////////////////////////////
// User Listing Packet
///////////////////////////////////////////////

pub struct UserListingPacket{
    pub room: String,
    pub users: Vec<String>,
}

impl UserListingPacket {

    pub fn new() -> Result<'static, Self> {
            Ok(UserListingPacket {
                users: Vec::new(),
                room: "Unknown".to_string(),
                })
    }

    pub fn from_room_and_vec(new_room: &String, new_users: &Vec<String>) -> Result<'static, Self> {
            Ok(UserListingPacket {
                users: new_users.to_owned(),
                room: new_room.to_owned(),
                })
    }

    pub fn push(&mut self, user: &String) -> Result<()> {
        let a_user = valid_name(&user)?.to_owned();
        self.users.push(a_user.to_owned());
        Ok(())
    }

    pub fn set_room(&mut self, new_room: &String) -> Result<()> {
        let a_room = valid_name(&new_room)?.to_owned();
        self.room = new_room.to_owned();
        Ok(())
    }


}

impl IrcPacket for UserListingPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(133);
        bytes_out.put_u8( IrcKind::IRC_KIND_USER_LISTING as u8);
        bytes_out.put_u32(64 + (64 * self.users.len()) as u32);

        bytes_out.put_slice(&self.room.as_bytes());
        let remain = 64 - self.room.len();
        bytes_out.put_bytes(b'\0', remain);
        for user in &self.users{
            bytes_out.put_slice(&user.as_bytes());
            let remain = 64 - user.len();
            for x in 1..remain+1 {
                bytes_out.put_u8(b'\0');
            }
        }
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_USER_LISTING {
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);

        if length < 128 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let count_users: usize = ((length / 64) - 1) as usize;

        println!("blah{}", count_users);
        if source.len() as usize != 5 + 64 + (count_users*64) {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 5 + 64 + (count_users*64)));
        }

        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_room: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        let mut new_users: Vec<String> = Vec::new();

        for offset in 0..count_users {
            let new_username =  valid_name(&name_from_slice(&source[(offset*64)+5+64..((offset+1)*64)+5+64])?)?.to_owned();
            new_users.push(new_username);
        }

        Ok(UserListingPacket {
          users: new_users,
          room: new_room,
        })
    }
}


///////////////////////////////////////////////
// Query User Packet
///////////////////////////////////////////////

    #[allow(non_camel_case_types)]
    #[derive(Copy,Clone,FromPrimitive,PartialEq)]
    #[repr(u8)]
    pub enum UserStatus {
        Online = 0x01,
        Offline = 0x00,
        Request = 0x02,

        #[num_enum(default)]
        NO_MATCH_USER_STATUS,
    }

pub struct QueryUserPacket {

    pub user_name: String,
    pub status: UserStatus,
}

impl QueryUserPacket {

    pub fn new(username: & String) -> Result<Self> {
            let v_username = valid_name(username)?;
            Ok(QueryUserPacket {
                        user_name: v_username.to_owned(),
                        status: UserStatus::Request,
            })
    }

}

impl IrcPacket for QueryUserPacket {

    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(70);
        bytes_out.put_u8( IrcKind::IRC_KIND_QUERY_USER as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.user_name.as_bytes());
        let remain = 64 - self.user_name.len();
        for x in 1..remain+1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out.put_u8( self.status as u8);
        bytes_out
    }

    fn from_bytes(source: &[u8] ) -> Result<Self> {
        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let kind_raw= IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_QUERY_USER{
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_username =  valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_user_status: UserStatus = UserStatus::from(source[5]);
        match new_user_status {
            UserStatus::NO_MATCH_USER_STATUS => Err(IrcError::CodeOutOfRange()),
            user_status=> Ok(QueryUserPacket {
                            user_name: new_username,
                            status: user_status,
                            })
        }
    }
}

#[cfg(test)]
#[path = "./lib/test.rs"]
mod irclib;
