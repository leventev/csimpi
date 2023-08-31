use std::io::prelude::*;
use std::{net::TcpStream, str::from_utf8};

#[repr(C)]
struct PacketHeader {
    magic: u32,
    packet_type: u32,
}

#[derive(Debug)]
pub enum PacketPayload {
    Connect(String),
    ConnectResponse(bool),
    SendMessage(String),
    Message(String, String),
}

#[derive(Debug)]
pub enum PacketError {
    FailedToReadPacket,
    InvalidSize,
    InvalidMagic,
    InvalidPayload,
    InvalidPacketType,
}

const PACKET_MAGIC: u32 = 0xCAFEBABE;
const HEADER_SIZE: usize = core::mem::size_of::<PacketHeader>();
const MAX_PACKET_SIZE: usize = 1024;

const CONNECT_TYPE: u32 = 0;
const CONNECT_RESPONSE_TYPE: u32 = 1;
const CONNECT_SEND_MESSAGE: u32 = 2;
const CONNECT_MESSAGE: u32 = 3;

pub fn read_packet(stream: &mut TcpStream) -> Result<PacketPayload, PacketError> {
    let mut buff = [0u8; MAX_PACKET_SIZE];
    match stream.read(&mut buff[..]) {
        Ok(read) => parse_packet(&buff[..read]),
        Err(_) => Err(PacketError::FailedToReadPacket),
    }
}

pub fn parse_packet(buff: &[u8]) -> Result<PacketPayload, PacketError> {
    if buff.len() < HEADER_SIZE {
        return Err(PacketError::InvalidSize);
    }

    // TODO: parse with copy due to endianness
    let header = unsafe { (buff.as_ptr() as *const PacketHeader).as_ref() }.unwrap();
    if header.magic != PACKET_MAGIC {
        return Err(PacketError::InvalidMagic);
    }

    let payload_buff = &buff[HEADER_SIZE..];
    PacketPayload::parse(header.packet_type, payload_buff)
}

impl PacketPayload {
    fn parse(payload_type: u32, buff: &[u8]) -> Result<PacketPayload, PacketError> {
        match payload_type {
            CONNECT_TYPE => Self::parse_connect(buff),
            CONNECT_RESPONSE_TYPE => Self::parse_connect_response(buff),
            _ => Err(PacketError::InvalidPacketType),
        }
    }

    fn parse_connect(buff: &[u8]) -> Result<PacketPayload, PacketError> {
        if buff.len() < 2 {
            return Err(PacketError::InvalidPayload);
        }

        let username_len = buff[0] as usize;
        if buff.len() != 1 + username_len {
            return Err(PacketError::InvalidPayload);
        }

        let username = match from_utf8(&buff[1..]) {
            Ok(username) => username,
            Err(_) => return Err(PacketError::InvalidPayload),
        };

        Ok(PacketPayload::Connect(username.to_string()))
    }

    fn parse_connect_response(buff: &[u8]) -> Result<PacketPayload, PacketError> {
        if buff.len() != 1 {
            return Err(PacketError::InvalidPayload);
        }

        let val: bool = buff[0] == 1;

        Ok(PacketPayload::ConnectResponse(val))
    }

    fn packet_type(&self) -> u32 {
        match self {
            Self::Connect(_) => 0,
            Self::ConnectResponse(_) => 1,
            Self::SendMessage(_) => 2,
            Self::Message(_, _) => 3,
        }
    }

    pub fn create_packet(&self) -> Vec<u8> {
        let buff = match self {
            Self::Connect(name) => Self::create_connect_packet(name),
            Self::ConnectResponse(success) => Self::create_connect_response_packet(*success),
            _ => todo!(),
        };

        let header = unsafe { (buff.as_ptr() as *mut PacketHeader).as_mut() }.unwrap();

        header.magic = PACKET_MAGIC;
        header.packet_type = self.packet_type();

        buff
    }

    fn create_connect_packet(name: &str) -> Vec<u8> {
        assert!(name.len() < 256);
        let mut buff = vec![0u8; HEADER_SIZE + name.len() + 1];
        let payload_buff = &mut buff[HEADER_SIZE..];

        payload_buff[0] = name.len() as u8;
        payload_buff[1..].copy_from_slice(name.as_bytes());

        buff
    }

    fn create_connect_response_packet(success: bool) -> Vec<u8> {
        let mut buff = vec![0u8; HEADER_SIZE + 1];
        let payload_buff = &mut buff[HEADER_SIZE..];

        payload_buff[0] = success.into();

        buff
    }
}
