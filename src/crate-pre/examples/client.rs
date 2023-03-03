use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

fn main() {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    tcp_stream.write_all(b"hello yivei!").unwrap();
    let mut buf = [0u8; 17];

    tcp_stream.read_exact(&mut buf).unwrap();
    println!("{:?}", String::from_utf8_lossy(&buf));
}
