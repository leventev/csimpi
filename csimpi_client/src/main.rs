use clap::Parser;
use std::{io::Write, net::TcpStream};

use csimpi_protocol::{PacketError, PacketPayload};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(long)]
    address: String,

    #[arg(long)]
    username: String,
}

#[derive(Debug)]
enum NetworkError {
    TcpError,
    WriteFailed,
    ReadFailed(PacketError),
    ConnectionFailed,
    InvalidPacket,
}

fn main() {
    let args = Args::parse();

    assert!(!args.address.is_empty());
    assert!(!args.username.is_empty());

    let stream = connect(&args.address, &args.username).expect("Failed to connect to server");
    println!("successfully connected to {}", args.address);
}

fn connect(address: &str, username: &str) -> Result<TcpStream, NetworkError> {
    let mut stream = match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(_) => return Err(NetworkError::TcpError),
    };

    let connect = PacketPayload::Connect(username.to_string());
    let packet = connect.create_packet();
    if stream.write_all(&packet[..]).is_err() {
        return Err(NetworkError::WriteFailed);
    }

    let res = match csimpi_protocol::read_packet(&mut stream) {
        Ok(payload) => payload,
        Err(err) => return Err(NetworkError::ReadFailed(err)),
    };

    match res {
        PacketPayload::ConnectResponse(true) => Ok(stream),
        PacketPayload::ConnectResponse(false) => Err(NetworkError::ConnectionFailed),
        _ => Err(NetworkError::InvalidPacket),
    }
}
