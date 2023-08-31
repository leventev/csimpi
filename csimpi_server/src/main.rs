use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use csimpi_protocol::PacketError;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), PacketError> {
    let mut buff = [0; 1024];
    match stream.read(&mut buff) {
        Ok(n) => handle_packet(&buff[..n]),
        Err(_) => return Err(PacketError::FailedToReadPacket),
    }
}

fn handle_packet(buff: &[u8]) -> Result<(), PacketError> {
    let payload = csimpi_protocol::parse_packet(buff)?;

    match payload {
        csimpi_protocol::PacketPayload::Connect(username) => {
            println!("{} connected", username);
            Ok(())
        }
        _ => {
            println!("invalid packet type");
            Err(PacketError::InvalidPacketType)
        }
    }
}
