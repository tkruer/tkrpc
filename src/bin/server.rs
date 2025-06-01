use std::net::TcpListener;
use tkrpc::server::{SERVER_ADDR, handle_client};

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", SERVER_ADDR, e);
        std::process::exit(1);
    });

    println!("Server listening on {}", SERVER_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Incoming request with stream");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
