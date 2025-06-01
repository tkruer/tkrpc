use std::io::{self, Read, Write};
use std::net::TcpStream;

const SERVER_ADDR: &str = "localhost:5005";

pub fn pack(input: u16) -> [u8; 2] {
    input.to_be_bytes() // Convert to network byte order (big-endian)
}

pub fn get_socket() -> io::Result<TcpStream> {
    TcpStream::connect(SERVER_ADDR)
}

pub fn is_prime_rpc(number: u16) -> bool {
    let packed = pack(number);

    // Connect to the server
    let mut stream = match get_socket() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("client: failed to connect - {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = stream.write_all(&packed) {
        eprintln!("send error: {}", e);
        std::process::exit(1);
    }

    let mut buf = [0u8; 1];
    if let Err(e) = stream.read_exact(&mut buf) {
        eprintln!("recv error: {}", e);
        std::process::exit(1);
    }

    buf[0] != 0
}
