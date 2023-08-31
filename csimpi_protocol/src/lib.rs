use std::str::from_utf8;

#[repr(C)]
struct PacketHeader {
    magic: u32,
    packet_type: u32,
}

#[derive(Debug)]
pub enum PacketPayload<'a> {
    Connect(&'a str),
    SendMessage(&'a str),
    Message(&'a str, &'a str),
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

pub fn parse_packet<'a>(buff: &'a [u8]) -> Result<PacketPayload<'a>, PacketError> {
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

impl<'a> PacketPayload<'a> {
    fn parse(payload_type: u32, buff: &'a [u8]) -> Result<PacketPayload<'a>, PacketError> {
        match payload_type {
            0 => Self::parse_connect(buff),
            _ => Err(PacketError::InvalidPacketType),
        }
    }

    fn parse_connect(buff: &'a [u8]) -> Result<PacketPayload<'a>, PacketError> {
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

        Ok(PacketPayload::Connect(username))
    }

    fn packet_type(&self) -> u32 {
        match self {
            Self::Connect(_) => 0,
            Self::SendMessage(_) => 1,
            Self::Message(_, _) => 2,
        }
    }

    pub fn create_packet(&self) -> Vec<u8> {
        let buff = match self {
            Self::Connect(name) => Self::create_connect_packet(name),
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
}
