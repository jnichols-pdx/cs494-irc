#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use bytes::{Buf, BufMut, Bytes, BytesMut};
use duplicate::duplicate;
use lazy_static::lazy_static;
use num_enum::FromPrimitive;
use regex::Regex;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::fmt;
use thiserror::Error;

/// Result type for IRC  errors.
pub type Result<'a, T> = std::result::Result<T, IrcError>;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
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
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
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

    #[error("Code out of range")]
    CodeOutOfRange(),

    //Wrappers around library errors we may encounter
    #[error("Encountered IO Error: {0}")]
    Io(io::Error),

    #[error("Encountered FromUTF8 Error: {0}")]
    FromUtf8Err(std::string::FromUtf8Error),

    #[error("Encountered UTF8 Error: {0}")]
    Utf8Err(std::str::Utf8Error),

    #[error("Encountered Join Error: {0}")]
    JoinErr(tokio::task::JoinError),

    #[error("Encountered mpsc Send error: {0}")]
    SendPackErr(tokio::sync::mpsc::error::SendError<SyncSendPack>),
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

impl From<tokio::task::JoinError> for IrcError {
    fn from(err: tokio::task::JoinError) -> IrcError {
        IrcError::JoinErr(err)
    }
}

impl From<tokio::sync::mpsc::error::SendError<SyncSendPack>> for IrcError {
    fn from(err: tokio::sync::mpsc::error::SendError<SyncSendPack>) -> IrcError {
        IrcError::SendPackErr(err)
    }
}

///////////////////////////////////////////////
// UTILITY functions
///////////////////////////////////////////////

pub fn valid_name<'a>(name: &'a String) -> Result<&'a String> {
    //NAMES
    //must be 64 bytes or less in utf-8 encoding,
    //must be more than 0 and less than 32 codepoints,
    //must not have 0x00-0x1f (low ascii control codes),
    //must not have 0x20 (space character)
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)

    let byte_size = name.len();
    if byte_size > 64 {
        return Err(IrcError::TooManyBytes(byte_size, 64));
    }

    let num_points = name.chars().count();
    if num_points == 0 {
        return Err(IrcError::InvalidEmpty());
    }
    if num_points > 32 {
        return Err(IrcError::NameTooLong(num_points));
    }

    lazy_static! {
        static ref REN: Regex = Regex::new(
            "[\u{00}-\u{1F}\u{20}\u{202A}-\u{202E}\u{2066}-\u{2069}\u{200E}\u{200F}\u{061C}]"
        )
        .unwrap();
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
    //must not have 0x00-0x19 (low ascii control codes),
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)
    //must have \00 as the final byte.

    let byte_size = message.len();
    if byte_size > 12000 {
        return Err(IrcError::TooManyBytes(byte_size, 64));
    }

    let num_points = message.chars().count();
    if num_points <= 1 {
        return Err(IrcError::InvalidEmpty());
    }

    lazy_static! {
        static ref REM: Regex = Regex::new("[\u{01}-\u{08}\u{0A}-\u{1F}\u{202A}-\u{202E}\u{2066}-\u{2069}\u{200E}\u{200F}\u{061C}]").unwrap();
    }
    if REM.is_match(message) {
        return Err(IrcError::InvalidMessageContent());
    }

    match message.find('\x00') {
        None => {
            return Err(IrcError::InvalidMessageContent());
        } //must end with one
        Some(pos) => {
            if pos != byte_size - 1 {
                //may not appear before the end
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
    //must not have 0x00-0x19 (low ascii control codes),
    //must not have 0x202A-0x202E, 0x2066-0x2069, 0x200E, 0x200F or 0x061C (directional codes)
    //must not have 0x003A or 0x002F (file system path delimiters ':' and '/')

    let byte_size = file_name.len();
    if byte_size > 1024 {
        return Err(IrcError::TooManyBytes(byte_size, 1024));
    }

    let num_points = file_name.chars().count();
    if num_points == 0 {
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

pub fn get_four_bytes_as_array(source: &[u8]) -> [u8; 4] {
    source.try_into().expect("Slice with incorrect length.")
}

pub fn u32_from_slice(source: &[u8]) -> u32 {
    u32::from_be_bytes(get_four_bytes_as_array(&source[0..4]))
}

pub fn get_two_bytes_as_array(source: &[u8]) -> [u8; 2] {
    source.try_into().expect("Slice with incorrect length.")
}

pub fn u16_from_slice(source: &[u8]) -> u16 {
    u16::from_be_bytes(get_two_bytes_as_array(&source[0..2]))
}

pub fn get_sixtyfour_bytes_as_array(source: &[u8]) -> [u8; 64] {
    source.try_into().expect("Slice with incorrect length.")
}

pub fn name_from_slice(source: &[u8]) -> Result<String> {
    if source.len() != 64 {
        return Err(IrcError::PacketLengthIncorrect(source.len(), 64));
    }
    match String::from_utf8(get_sixtyfour_bytes_as_array(&source[..]).to_vec()) {
        Ok(mut n) => match n.find('\0') {
            Some(pos) => {
                n.truncate(pos);
                Ok(n)
            }
            None => Ok(n),
        },
        Err(e) => Err(IrcError::FromUtf8Err(e)),
    }
}

pub fn string_from_slice(source: &[u8]) -> Result<String> {
    let string_str = std::str::from_utf8(source)?;
    let new_string: String = string_str.into();
    Ok(new_string)
}

pub trait IrcPacket {
    fn as_bytes(&self) -> BytesMut;

    fn from_bytes(source: &[u8]) -> Result<Self>
    where
        Self: Sized;

}

/////////////////////////////
// Ugly Passing Struct
/////////////////////////////

//Rust's requirements for moving data between threads require said data to implement
//both SYNC and SEND traits. All of the concrete implementations of my IrcPacket trait
//ARE SYNC and SEND, however I haven't found a way to mark a *trait* as SYNC/SEND...
//Which is reasonable as we have no gaurantees that some other user wouldn't make their 
//own concrete implementation of IrcPacket that was NOT Sync and Send.
//
//This means that if I cannot stuff an 'IrcPacket' through an MPSC channel. I can instead
//either set the channel up for a single concrete packet type or some other concrete object
//type, such as this SyncSendPack struct.
//
//This ugly struct simply wraps all the potential concrete implementations of IrcPacket into
//a single struct, which itself is SYNC/SEND and thus can between tasks/threads via tokio or std
//channels.
#[derive(Debug)]
pub struct SyncSendPack {
    pub contained_kind: IrcKind,
    pub errp: Option<ErrorPacket>,
    pub ncp: Option<NewClientPacket>,
    pub hbp: Option<HeartbeatPacket>,
    pub erp: Option<EnterRoomPacket>,
    pub lrp: Option<LeaveRoomPacket>,
    pub lip: Option<ListRoomsPacket>,
    pub rlp: Option<RoomListingPacket>,
    pub ulp: Option<UserListingPacket>,
    pub qup: Option<QueryUserPacket>,
    pub smp: Option<SendMessagePacket>,
    pub bmp: Option<BroadcastMessagePacket>,
    pub pmp: Option<PostMessagePacket>,
    pub dmp: Option<DirectMessagePacket>,
    pub ofp: Option<OfferFilePacket>,
    pub afp: Option<AcceptFilePacket>,
    pub rfp: Option<RejectFilePacket>,
    pub ftp: Option<FileTransferPacket>,
    pub cdp: Option<ClientDepartsPacket>,
    pub sdp: Option<ServerDepartsPacket>,
}

impl From<ErrorPacket> for SyncSendPack {
    fn from(packet_in: ErrorPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_ERR,
            errp: Some(packet_in),
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<NewClientPacket> for SyncSendPack {
    fn from(packet_in: NewClientPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_NEW_CLIENT,
            errp: None,
            ncp: Some(packet_in),
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<HeartbeatPacket> for SyncSendPack {
    fn from(packet_in: HeartbeatPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_HEARTBEAT,
            errp:None,
            ncp: None,
            hbp: Some(packet_in),
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<EnterRoomPacket> for SyncSendPack {
    fn from(packet_in: EnterRoomPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_ENTER_ROOM,
            errp:None,
            ncp: None,
            hbp: None,
            erp: Some(packet_in),
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<LeaveRoomPacket> for SyncSendPack {
    fn from(packet_in: LeaveRoomPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_LEAVE_ROOM,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: Some(packet_in),
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<ListRoomsPacket> for SyncSendPack {
    fn from(packet_in: ListRoomsPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_LIST_ROOMS,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: Some(packet_in),
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<RoomListingPacket> for SyncSendPack {
    fn from(packet_in: RoomListingPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_ROOM_LISTING,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: Some(packet_in),
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<UserListingPacket> for SyncSendPack {
    fn from(packet_in: UserListingPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_USER_LISTING,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: Some(packet_in),
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<QueryUserPacket> for SyncSendPack {
    fn from(packet_in: QueryUserPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_QUERY_USER,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: Some(packet_in),
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<SendMessagePacket> for SyncSendPack {
    fn from(packet_in: SendMessagePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_SEND_MESSAGE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: Some(packet_in),
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<BroadcastMessagePacket> for SyncSendPack {
    fn from(packet_in: BroadcastMessagePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_BROADCAST_MESSAGE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: Some(packet_in),
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<PostMessagePacket> for SyncSendPack {
    fn from(packet_in: PostMessagePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_POST_MESSAGE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: Some(packet_in),
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<DirectMessagePacket> for SyncSendPack {
    fn from(packet_in: DirectMessagePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_DIRECT_MESSAGE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: Some(packet_in),
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<OfferFilePacket> for SyncSendPack {
    fn from(packet_in: OfferFilePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_OFFER_FILE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: Some(packet_in),
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<AcceptFilePacket> for SyncSendPack {
    fn from(packet_in: AcceptFilePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_ACCEPT_FILE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: Some(packet_in),
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<RejectFilePacket> for SyncSendPack {
    fn from(packet_in: RejectFilePacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_REJECT_FILE,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: Some(packet_in),
            ftp: None,
            cdp: None,
            sdp: None,
        }
    }
}

impl From<FileTransferPacket> for SyncSendPack {
    fn from(packet_in: FileTransferPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_FILE_TRANSFER,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: Some(packet_in),
            cdp: None,
            sdp: None,
        }
    }
}

impl From<ClientDepartsPacket> for SyncSendPack {
    fn from(packet_in: ClientDepartsPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_CLIENT_DEPARTS,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: Some(packet_in),
            sdp: None,
        }
    }
}

impl From<ServerDepartsPacket> for SyncSendPack {
    fn from(packet_in: ServerDepartsPacket) -> SyncSendPack {
        SyncSendPack {
            contained_kind: IrcKind::IRC_KIND_SERVER_DEPARTS,
            errp:None,
            ncp: None,
            hbp: None,
            erp: None,
            lrp: None,
            lip: None,
            rlp: None,
            ulp: None,
            qup: None,
            smp: None,
            bmp: None,
            pmp: None,
            dmp: None,
            ofp: None,
            afp: None,
            rfp: None,
            ftp: None,
            cdp: None,
            sdp: Some(packet_in),
        }
    }
}

///////////////////////////////////////////////
//  Error Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct ErrorPacket {
    pub error_code: IrcErrCode,
}

impl ErrorPacket {
    pub fn new(code: IrcErrCode) -> Result<'static, Self> {
        Ok(ErrorPacket {
            error_code: code.to_owned(),
        })
    }
}

impl IrcPacket for ErrorPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8(IrcKind::IRC_KIND_ERR as u8);
        bytes_out.put_u32(1);
        bytes_out.put_u8(self.error_code as u8);
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ERR {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 6 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 6));
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 1 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_error_code: IrcErrCode = IrcErrCode::from(source[5]);
        match new_error_code {
            IrcErrCode::NO_MATCH_IRC_ERR => Err(IrcError::CodeOutOfRange()),
            code => Ok(ErrorPacket { error_code: code }),
        }
    }
}

///////////////////////////////////////////////
// NewClient Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct NewClientPacket {
    pub chat_name: String,
}

impl NewClientPacket {
    pub fn new(name: &String) -> Result<Self> {
        let v_name = valid_name(name)?;
        Ok(NewClientPacket {
            chat_name: v_name.to_owned(),
        })
    }
}

impl IrcPacket for NewClientPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8(IrcKind::IRC_KIND_NEW_CLIENT as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.chat_name.as_bytes());
        let remain = 64 - self.chat_name.len();
        for x in 1..remain + 1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_NEW_CLIENT {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_name = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(NewClientPacket {
            chat_name: new_name,
        })
    }
}

///////////////////////////////////////////////
// Heartbeat Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct HeartbeatPacket {}

impl HeartbeatPacket {
    pub fn new() -> Result<'static, Self> {
        Ok(HeartbeatPacket {})
    }
}

impl IrcPacket for HeartbeatPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(5);
        bytes_out.put_u8(IrcKind::IRC_KIND_HEARTBEAT as u8);
        bytes_out.put_u32(0);
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_HEARTBEAT {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 5));
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

#[derive(Debug)]
pub struct EnterRoomPacket {
    pub room_name: String,
}

impl EnterRoomPacket {
    pub fn new(roomname: &String) -> Result<Self> {
        let v_roomname = valid_name(roomname)?;
        Ok(EnterRoomPacket {
            room_name: v_roomname.to_owned(),
        })
    }
}

impl IrcPacket for EnterRoomPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8(IrcKind::IRC_KIND_ENTER_ROOM as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.room_name.as_bytes());
        let remain = 64 - self.room_name.len();
        for x in 1..remain + 1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ENTER_ROOM {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_roomname = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(EnterRoomPacket {
            room_name: new_roomname,
        })
    }
}

///////////////////////////////////////////////
// Leave Room Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct LeaveRoomPacket {
    pub room_name: String,
}

impl LeaveRoomPacket {
    pub fn new(roomname: &String) -> Result<Self> {
        let v_roomname = valid_name(roomname)?;
        Ok(LeaveRoomPacket {
            room_name: v_roomname.to_owned(),
        })
    }
}

impl IrcPacket for LeaveRoomPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(69);
        bytes_out.put_u8(IrcKind::IRC_KIND_LEAVE_ROOM as u8);
        bytes_out.put_u32(64);
        bytes_out.put_slice(&self.room_name.as_bytes());
        let remain = 64 - self.room_name.len();
        for x in 1..remain + 1 {
            bytes_out.put_u8(b'\0');
        }
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_LEAVE_ROOM {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 69 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 69));
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_roomname = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        Ok(LeaveRoomPacket {
            room_name: new_roomname,
        })
    }
}

///////////////////////////////////////////////
// List Rooms Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct ListRoomsPacket {}

impl ListRoomsPacket {
    pub fn new() -> Result<'static, Self> {
        Ok(ListRoomsPacket {})
    }
}

impl IrcPacket for ListRoomsPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(5);
        bytes_out.put_u8(IrcKind::IRC_KIND_LIST_ROOMS as u8);
        bytes_out.put_u32(0);
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_LIST_ROOMS {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 5));
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

#[derive(Debug)]
pub struct RoomListingPacket {
    pub rooms: Vec<String>,
}

impl RoomListingPacket {
    pub fn new() -> Result<'static, Self> {
        Ok(RoomListingPacket { rooms: Vec::new() })
    }

    pub fn from_vec(new_rooms: &Vec<String>) -> Result<'static, Self> {
        Ok(RoomListingPacket {
            rooms: new_rooms.to_owned(), //takes ownership..?
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
        bytes_out.put_u8(IrcKind::IRC_KIND_ROOM_LISTING as u8);
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

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_ROOM_LISTING {
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);

        if length < 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }
        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let count_rooms: usize = ((length / 64) - 1) as usize;

        if source.len() as usize != (count_rooms * 64) + 5 + 64 {
            return Err(IrcError::PacketLengthIncorrect(
                source.len(),
                (count_rooms * 64) + 5 + 64,
            ));
        }

        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        //Ignores bytes 5..69 which are the unused identifier field.
        let mut new_rooms: Vec<String> = Vec::new();

        for offset in 0..count_rooms {
            let new_roomname = valid_name(&name_from_slice(
                &source[(offset * 64) + 5 + 64..((offset + 1) * 64) + 5 + 64],
            )?)?
            .to_owned();
            new_rooms.push(new_roomname);
        }

        Ok(RoomListingPacket { rooms: new_rooms })
    }
}

///////////////////////////////////////////////
// User Listing Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct UserListingPacket {
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
        bytes_out.put_u8(IrcKind::IRC_KIND_USER_LISTING as u8);
        bytes_out.put_u32(64 + (64 * self.users.len()) as u32);

        bytes_out.put_slice(&self.room.as_bytes());
        let remain = 64 - self.room.len();
        bytes_out.put_bytes(b'\0', remain);
        for user in &self.users {
            bytes_out.put_slice(&user.as_bytes());
            let remain = 64 - user.len();
            for x in 1..remain + 1 {
                bytes_out.put_u8(b'\0');
            }
        }
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_USER_LISTING {
            return Err(IrcError::PacketMismatch());
        }

        let length = u32_from_slice(&source[1..5]);

        if length < 64 {
            return Err(IrcError::FieldLengthIncorrect());
        }
        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let count_users: usize = ((length / 64) - 1) as usize;

        if source.len() as usize != 5 + 64 + (count_users * 64) {
            return Err(IrcError::PacketLengthIncorrect(
                source.len(),
                5 + 64 + (count_users * 64),
            ));
        }

        if length % 64 != 0 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_room: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();
        let mut new_users: Vec<String> = Vec::new();

        for offset in 0..count_users {
            let new_username = valid_name(&name_from_slice(
                &source[(offset * 64) + 5 + 64..((offset + 1) * 64) + 5 + 64],
            )?)?
            .to_owned();
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
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(u8)]
pub enum UserStatus {
    Online = 0x01,
    Offline = 0x00,
    Request = 0x02,

    #[num_enum(default)]
    NO_MATCH_USER_STATUS,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match *self {
        UserStatus::Online => write!(f, "Online"),
        UserStatus::Offline => write!(f, "Offline"),
        UserStatus::Request => write!(f, "Status Requested"),
        UserStatus::NO_MATCH_USER_STATUS => write!(f, "Unknown"),
       }
    }
}

#[derive(Debug)]
pub struct QueryUserPacket {
    pub user_name: String,
    pub status: UserStatus,
}

impl QueryUserPacket {
    pub fn new(username: &String) -> Result<Self> {
        let v_username = valid_name(username)?;
        Ok(QueryUserPacket {
            user_name: v_username.to_owned(),
            status: UserStatus::Request,
        })
    }

    pub fn set_online(&mut self) {
        self.status = UserStatus::Online;
    }

    pub fn set_offline(&mut self) {
        self.status = UserStatus::Offline;
    }
    pub fn set_query(&mut self) {
        self.status = UserStatus::Request;
    }
}

impl IrcPacket for QueryUserPacket {
    fn as_bytes(&self) -> BytesMut {
        let mut bytes_out = BytesMut::with_capacity(70);
        bytes_out.put_u8(IrcKind::IRC_KIND_QUERY_USER as u8);
        bytes_out.put_u32(65);
        bytes_out.put_slice(&self.user_name.as_bytes());
        let remain = 64 - self.user_name.len();
        bytes_out.put_bytes(b'\0', remain);
        bytes_out.put_u8(self.status as u8);
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_QUERY_USER {
            return Err(IrcError::PacketMismatch());
        }

        if source.len() != 70 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), 70));
        }

        let length = u32_from_slice(&source[1..5]);
        if length != 65 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        let new_username = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_user_status: UserStatus = UserStatus::from(source[69]); match new_user_status {
            UserStatus::NO_MATCH_USER_STATUS => Err(IrcError::CodeOutOfRange()),
            user_status => Ok(QueryUserPacket {
                user_name: new_username,
                status: user_status,
            }),
        }
    }
}

///////////////////////////////////////////////
// Send Message Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct SendMessagePacket {
    pub room: String,
    pub message: String,
}

impl SendMessagePacket {
    pub fn new<'x>(to_room: &String, message: &String) -> Result<'x, SendMessagePacket> {
        let v_room = valid_name(to_room)?;
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(SendMessagePacket {
            room: v_room.to_owned(),
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for SendMessagePacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + 64 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_SEND_MESSAGE as u8);
        bytes_out.put_u32(64 + (message_bytelength as u32));
        bytes_out.put_slice(&self.room.as_bytes());
        let remain = 64 - self.room.len();
        bytes_out.put_bytes(b'\0', remain);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_SEND_MESSAGE {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 70 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_room: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_message = valid_message(&String::from_utf8(source[69..].to_vec())?)?.to_string();

        Ok(SendMessagePacket {
            room: new_room,
            message: new_message,
        })
    }
}

///////////////////////////////////////////////
// Broadcast Message Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct BroadcastMessagePacket {
    pub message: String,
}

impl BroadcastMessagePacket {
    pub fn new(message: &String) -> Result<BroadcastMessagePacket> {
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(BroadcastMessagePacket {
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for BroadcastMessagePacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_BROADCAST_MESSAGE as u8);
        bytes_out.put_u32(message_bytelength as u32);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_BROADCAST_MESSAGE {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 1 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_message = valid_message(&String::from_utf8(source[5..].to_vec())?)?.to_string();

        Ok(BroadcastMessagePacket {
            message: new_message,
        })
    }
}

///////////////////////////////////////////////
// Post Message Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct PostMessagePacket {
    pub room: String,
    pub sender: String,
    pub message: String,
}

impl PostMessagePacket {
    pub fn new<'x>(
        to_room: &String,
        from_user: &String,
        message: &String,
    ) -> Result<'x, PostMessagePacket> {
        let v_room = valid_name(to_room)?;
        let v_sender = valid_name(from_user)?;
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(PostMessagePacket {
            room: v_room.to_owned(),
            sender: v_sender.to_owned(),
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for PostMessagePacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + 64 + 64 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_POST_MESSAGE as u8);
        bytes_out.put_u32(64 + 64 + (message_bytelength as u32));
        bytes_out.put_slice(&self.room.as_bytes());
        let remain = 64 - self.room.len();
        bytes_out.put_bytes(b'\0', remain);
        bytes_out.put_slice(&self.sender.as_bytes());
        let remain = 64 - self.sender.len();
        bytes_out.put_bytes(b'\0', remain);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_POST_MESSAGE {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 134 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_room: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_sender: String = valid_name(&name_from_slice(&source[69..133])?)?.to_owned();

        let new_message = valid_message(&String::from_utf8(source[133..].to_vec())?)?.to_string();

        Ok(PostMessagePacket {
            room: new_room,
            sender: new_sender,
            message: new_message,
        })
    }
}

///////////////////////////////////////////////
// Direct Message Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct DirectMessagePacket {
    pub target: String,
    pub message: String,
}

impl DirectMessagePacket {
    pub fn new<'x>(to_target: &String, message: &String) -> Result<'x, DirectMessagePacket> {
        let v_target = valid_name(to_target)?;
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(DirectMessagePacket {
            target: v_target.to_owned(),
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for DirectMessagePacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + 64 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_DIRECT_MESSAGE as u8);
        bytes_out.put_u32(64 + (message_bytelength as u32));
        bytes_out.put_slice(&self.target.as_bytes());
        let remain = 64 - self.target.len();
        bytes_out.put_bytes(b'\0', remain);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_DIRECT_MESSAGE {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 70 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_target: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_message = valid_message(&String::from_utf8(source[69..].to_vec())?)?.to_string();

        Ok(DirectMessagePacket {
            target: new_target,
            message: new_message,
        })
    }
}

///////////////////////////////////////////////
// FILE TRANSFER HANDSHAKE PACKETS
///////////////////////////////////////////////

#[derive(Debug)]
pub struct TransferCore {
    pub recipient: String,
    pub sender: String,
    pub transfer_id: u16,
    pub file_size: u32,
    pub file_name: String,
}

#[derive(Debug)]
pub struct OfferFilePacket {
    core: TransferCore,
}

#[derive(Debug)]
pub struct AcceptFilePacket {
    core: TransferCore,
}

#[derive(Debug)]
pub struct RejectFilePacket {
    core: TransferCore,
}

impl TransferCore {
    pub fn new<'x>(
        to_user: &String,
        from_user: &String,
        size: u32,
        file_name: &String,
    ) -> Result<'x, TransferCore> {
        let v_recipient = valid_name(&to_user)?;
        let v_sender = valid_name(&from_user)?;
        let v_file_name = valid_filename(&file_name)?.to_owned();

        Ok(TransferCore {
            recipient: v_recipient.to_owned(),
            sender: v_sender.to_owned(),
            transfer_id: 0,
            file_size: size,
            file_name: v_file_name.to_owned(),
        })
    }

    pub fn set_id(&mut self, new_id: u16) {
        self.transfer_id = new_id;
    }

    pub fn byte_length(&self) -> usize {
        let filename_bytelength = self.file_name.len();
        return 64 + 64 + 2 + 4 + filename_bytelength;
    }

    fn as_bytes(&self) -> BytesMut {
        let content_length = self.byte_length();
        let mut bytes_out = BytesMut::with_capacity(content_length);

        bytes_out.put_slice(&self.recipient.as_bytes());
        let remain = 64 - self.recipient.len();
        bytes_out.put_bytes(b'\0', remain);

        bytes_out.put_slice(&self.sender.as_bytes());
        let remain = 64 - self.sender.len();
        bytes_out.put_bytes(b'\0', remain);

        bytes_out.put_u16(self.transfer_id);

        bytes_out.put_u32(self.file_size);

        bytes_out.put_slice(&self.file_name.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        //expects kind to still exist at front of buffer
        let kind_raw = IrcKind::from(source[0]);
        match kind_raw {
            IrcKind::IRC_KIND_OFFER_FILE
            | IrcKind::IRC_KIND_REJECT_FILE
            | IrcKind::IRC_KIND_ACCEPT_FILE => (),
            _ => return Err(IrcError::PacketMismatch()),
        };

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 135 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_recipient: String = valid_name(&name_from_slice(&source[5..69])?)?.to_owned();

        let new_sender: String = valid_name(&name_from_slice(&source[69..133])?)?.to_owned();

        let new_transfer_id = u16_from_slice(&source[133..135]);
        let new_file_size = u32_from_slice(&source[135..139]);
        let new_file_name =
            valid_filename(&String::from_utf8(source[139..].to_vec())?)?.to_string();

        Ok(TransferCore {
            recipient: new_recipient,
            sender: new_sender,
            transfer_id: new_transfer_id,
            file_size: new_file_size,
            file_name: new_file_name,
        })
    }
}

pub trait TransferCoreRead {
    fn get_to(&self) -> String;
    fn get_from(&self) -> String;
    fn get_file_name(&self) -> String;
    fn get_size(&self) -> u32;
    fn get_transfer_id(&self) -> u16;
    fn set_id(&mut self, new_id: u16);
    fn take_core(self) -> TransferCore;
}

#[duplicate(
    transfer_type;
    [OfferFilePacket];
    [AcceptFilePacket];
    [RejectFilePacket];
)]
impl TransferCoreRead for transfer_type {
    fn get_to(&self) -> String {
        let mut outgoing = self.core.recipient.clone();
        outgoing
    }

    fn get_from(&self) -> String {
        let mut outgoing = self.core.sender.clone();
        outgoing
    }

    fn get_file_name(&self) -> String {
        let mut outgoing = self.core.file_name.clone();
        outgoing
    }

    fn get_size(&self) -> u32 {
        self.core.file_size
    }

    fn get_transfer_id(&self) -> u16 {
        self.core.transfer_id
    }

    fn set_id(&mut self, new_id: u16) {
        self.core.set_id(new_id);
    }

    fn take_core(self) -> TransferCore {
        self.core
    }
}

#[duplicate(
    TRANSFER_TYPE        SPECIFIC_KIND                      PACKET_CLASS;
    [OfferFilePacket]    [IrcKind::IRC_KIND_OFFER_FILE]     [OfferFilePacket];
    [AcceptFilePacket]   [IrcKind::IRC_KIND_ACCEPT_FILE]    [AcceptFilePacket];
    [RejectFilePacket]   [IrcKind::IRC_KIND_REJECT_FILE]    [RejectFilePacket];
)]
impl IrcPacket for TRANSFER_TYPE {
    fn as_bytes(&self) -> BytesMut {
        let content_size = self.core.byte_length();
        let mut bytes_out = BytesMut::with_capacity(5 + content_size);
        bytes_out.put_u8(SPECIFIC_KIND as u8);
        bytes_out.put_u32(content_size as u32);
        bytes_out.put_slice(&self.core.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != SPECIFIC_KIND {
            return Err(IrcError::PacketMismatch());
        }

        Ok(PACKET_CLASS {
            core: TransferCore::from_bytes(source)?,
        })
    }
}

impl OfferFilePacket {
    pub fn new<'x>(
        to_user: &String,
        from_user: &String,
        size: u32,
        file_name: &String,
    ) -> Result<'x, OfferFilePacket> {
        Ok(OfferFilePacket {
            core: TransferCore::new(to_user, from_user, size, file_name)?,
        })
    }
}

impl AcceptFilePacket {
    pub fn new<'x>(
        to_user: &String,
        from_user: &String,
        transfer_id: u16,
        size: u32,
        file_name: &String,
    ) -> Result<'x, AcceptFilePacket> {
        let mut new_core = TransferCore::new(to_user, from_user, size, file_name)?;
        new_core.set_id(transfer_id);
        Ok(AcceptFilePacket { core: new_core })
    }

    pub fn from_offer<'x>(source: OfferFilePacket) -> Result<'x, AcceptFilePacket> {
        Ok(AcceptFilePacket {
            core: source.take_core(),
        })
    }
}

impl RejectFilePacket {
    pub fn new<'x>(
        to_user: &String,
        from_user: &String,
        transfer_id: u16,
        size: u32,
        file_name: &String,
    ) -> Result<'x, RejectFilePacket> {
        let mut new_core = TransferCore::new(to_user, from_user, size, file_name)?;
        new_core.set_id(transfer_id);
        Ok(RejectFilePacket { core: new_core })
    }

    pub fn from_offer<'x>(source: OfferFilePacket) -> Result<'x, RejectFilePacket> {
        Ok(RejectFilePacket {
            core: source.take_core(),
        })
    }
}

///////////////////////////////////////////////
// File Transfer Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct FileTransferPacket {
    pub transfer_id: u16,
    pub finished: bool,
    pub data: Bytes,
}
impl FileTransferPacket {
    pub fn new<'x>(
        transfer_id: u16,
        finished: bool,
        data: Bytes,
    ) -> Result<'x, FileTransferPacket> {
        if data.len() > 4096 {
            return Err(IrcError::TooManyBytes(data.len(), 4096));
        }
        if data.len() == 0 {
            return Err(IrcError::InvalidEmpty());
        }

        Ok(FileTransferPacket {
            transfer_id: transfer_id,
            finished: finished,
            data: data,
        })
    }
}

impl IrcPacket for FileTransferPacket {
    fn as_bytes(&self) -> BytesMut {
        let bytelength = self.data.len();
        let mut bytes_out = BytesMut::with_capacity(5 + 2 + 1 + bytelength);
        bytes_out.put_u8(IrcKind::IRC_KIND_FILE_TRANSFER as u8);
        bytes_out.put_u32(3 + bytelength as u32);
        bytes_out.put_u16(self.transfer_id);
        bytes_out.put_u8(self.finished as u8);
        bytes_out.put_slice(&self.data);
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_FILE_TRANSFER {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 4 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_transfer_id = u16_from_slice(&source[5..7]);
        let new_finished = &source[7] > &0;

        let new_data: Bytes = Bytes::copy_from_slice(&source[8..]);

        Ok(FileTransferPacket {
            transfer_id: new_transfer_id,
            finished: new_finished,
            data: new_data,
        })
    }
}

///////////////////////////////////////////////
// Client Departs Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct ClientDepartsPacket {
    pub message: String,
}

impl ClientDepartsPacket {
    pub fn new<'x>(message: &String) -> Result<'x, ClientDepartsPacket> {
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(ClientDepartsPacket {
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for ClientDepartsPacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_CLIENT_DEPARTS as u8);
        bytes_out.put_u32(message_bytelength as u32);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_CLIENT_DEPARTS {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 1 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_message = valid_message(&String::from_utf8(source[5..].to_vec())?)?.to_string();

        Ok(ClientDepartsPacket {
            message: new_message,
        })
    }
}

///////////////////////////////////////////////
// Server Departs Packet
///////////////////////////////////////////////

#[derive(Debug)]
pub struct ServerDepartsPacket {
    pub message: String,
}

impl ServerDepartsPacket {
    pub fn new<'x>(message: &String) -> Result<'x, ServerDepartsPacket> {
        let mut v_message;
        if message.ends_with('\0') {
            v_message = valid_message(&message)?.to_owned();
        } else {
            v_message = message.to_owned();
            v_message.push('\0');
            v_message = valid_message(&v_message)?.to_owned();
        }
        Ok(ServerDepartsPacket {
            message: v_message.to_owned(),
        })
    }

    pub fn get_message(&self) -> String {
        let mut outgoing = self.message.clone();
        outgoing.pop().unwrap();
        outgoing
    }
}

impl IrcPacket for ServerDepartsPacket {
    fn as_bytes(&self) -> BytesMut {
        let message_bytelength = self.message.len();
        let mut bytes_out = BytesMut::with_capacity(5 + (message_bytelength as usize));
        bytes_out.put_u8(IrcKind::IRC_KIND_SERVER_DEPARTS as u8);
        bytes_out.put_u32(message_bytelength as u32);
        bytes_out.put_slice(&self.message.as_bytes());
        bytes_out
    }

    fn from_bytes(source: &[u8]) -> Result<Self> {
        let kind_raw = IrcKind::from(source[0]);
        if kind_raw != IrcKind::IRC_KIND_SERVER_DEPARTS {
            return Err(IrcError::PacketMismatch());
        }

        let length: usize = u32_from_slice(&source[1..5]) as usize;

        if length < 1 {
            return Err(IrcError::FieldLengthIncorrect());
        }

        if source.len() != length + 5 {
            return Err(IrcError::PacketLengthIncorrect(source.len(), length + 5));
        }

        let new_message = valid_message(&String::from_utf8(source[5..].to_vec())?)?.to_string();

        Ok(ServerDepartsPacket {
            message: new_message,
        })
    }
}

#[cfg(test)]
#[path = "./lib/test.rs"]
mod irclib; //Names the block of tests we import. The *name* of this library is set in Cargo.toml
