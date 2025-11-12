use std::env;
use std::net::TcpStream;

static DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
static DEFAULT_SERVER_PORT: u16 = 4999;

fn connect(server_address: &str, port: u16) -> TcpStream {
    println!("Attempting to connect to server at {server_address}:{port}...");

    let stream = TcpStream::connect((server_address, port))
        .expect("Failed to connect to the server");

    println!("Successfully connected to the server");
    return stream;
}

fn main() {
    let server_address = env::args().nth(1).unwrap_or_else(|| DEFAULT_SERVER_ADDRESS.to_string());
    let port: u16 = env::args().nth(2)
        .unwrap_or_else(|| DEFAULT_SERVER_PORT.to_string())
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Invalid port number provided - using default {DEFAULT_SERVER_PORT}");
            DEFAULT_SERVER_PORT
        });

    let stream = connect(&server_address, port);
}
