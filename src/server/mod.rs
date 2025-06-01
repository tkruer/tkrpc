use crate::prime::is_prime;
use std::io::{Read, Write};
use std::net::TcpStream;

pub const SERVER_ADDR: &str = "0.0.0.0:5005";

pub fn unpack(input: [u8; 2]) -> u16 {
    u16::from_be_bytes(input)
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 2];
    if let Err(e) = stream.read_exact(&mut buf) {
        eprintln!("recv error: {}", e);
        return;
    }

    let number = unpack(buf);
    println!("Received a request: is {} prime?", number);

    let result = is_prime(number.try_into().unwrap());
    println!("Sending response: {}", result);

    let response = [result as u8];
    if let Err(e) = stream.write_all(&response) {
        eprintln!("send error: {}", e);
    }
}
