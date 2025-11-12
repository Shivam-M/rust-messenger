use std::env;

static DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
static DEFAULT_SERVER_PORT: u16 = 5000;

fn main() {
    let server_address = env::args().nth(1).unwrap_or_else(|| DEFAULT_SERVER_ADDRESS.to_string());
    let port: u16 = env::args().nth(2)
        .unwrap_or_else(|| DEFAULT_SERVER_PORT.to_string())
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Invalid port number provided - using default {DEFAULT_SERVER_PORT}");
            DEFAULT_SERVER_PORT
        });

    println!("Attempting to connect to server at {server_address}:{port}...");
}
