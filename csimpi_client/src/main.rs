use std::{io::Write, net::TcpStream};

use csimpi_protocol::{PacketError, PacketPayload};

#[derive(Debug)]
enum NetworkError {
    TcpError,
    WriteFailed,
    ReadFailed(PacketError),
    ConnectionFailed,
    InvalidPacket,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let address = args.get(1).expect("Address not specified");
    let username = args.get(2).expect("Username not specified");
    let stream = connect(address, username).expect("Failed to connect to server");

    println!("successfully connected to {}", address);
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
