use clap::{self, Parser};
use std::{
    fmt::Debug,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use csimpi_protocol::{PacketError, PacketPayload};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    address: String,
}

fn main() {
    let args = Args::parse();
    assert!(!args.address.is_empty());
    let listener = TcpListener::bind(args.address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), PacketError> {
    let mut buff = [0; 1024];
    match stream.read(&mut buff) {
        Ok(n) => handle_packet(&mut stream, &buff[..n]),
        Err(_) => Err(PacketError::FailedToReadPacket),
    }
}

fn handle_packet(stream: &mut TcpStream, buff: &[u8]) -> Result<(), PacketError> {
    let payload = csimpi_protocol::parse_packet(buff)?;

    match payload {
        csimpi_protocol::PacketPayload::Connect(username) => {
            println!("{} connected", username);

            let response = PacketPayload::ConnectResponse(true).create_packet();
            stream.write_all(&response[..]).unwrap();

            Ok(())
        }
        _ => {
            println!("invalid packet type");
            Err(PacketError::InvalidPacketType)
        }
    }
}
