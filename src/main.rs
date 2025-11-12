use std::env;
use std::net::TcpStream;
use std::io::Read;
use std::thread;
use serde_json::Value;

static DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
static DEFAULT_SERVER_PORT: u16 = 4999;
static BUFFER_SIZE: usize = 512;

static listening: bool = true;

fn listen(mut stream: &TcpStream) {
    println!("Listening to the server...");

    let mut buffer = [0; BUFFER_SIZE];

    while listening {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("No data received - likely server has closed the connection");
                break;
            }
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buffer[..n]).to_string();
                if let Ok(data) = serde_json::from_str::<Value>(&received_data) {
                    println!("Received data: {data}");
                } else {
                    eprintln!("Received malformed data from the server: {received_data}");
                }
            }
            Err(e) => {
                eprintln!("Error reading from the server: {e}");
                break;
            }
        }
    }
}

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

    thread::spawn(move || listen(&stream));

    loop {}
}
