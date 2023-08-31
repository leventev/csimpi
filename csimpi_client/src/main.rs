use std::{io::Write, net::TcpStream};

fn main() {
    let stream = connect();
}

fn connect() -> Result<TcpStream, ()> {
    let username = "test";
    let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();
    let connect = csimpi_protocol::PacketPayload::Connect(username);
    let packet = connect.create_packet();
    stream.write_all(&packet[..]).unwrap();
    Ok(stream)
}
