use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::{env, io};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use serde_json::Value;

static DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
static DEFAULT_SERVER_PORT: u16 = 4999;
static BUFFER_SIZE: usize = 512;

fn send(mut stream: &TcpStream, json_data: &serde_json::Value) {
    // println!("Sending message to server: {json_data}");

    stream.write_all(json_data.to_string().as_bytes())
        .expect(&format!("Failed to send data to the server"));
}

fn send_username(stream: &TcpStream, command: &str) {
    let username = match command.split_whitespace().nth(1) {
        Some(u) => u,
        None => {
            println!("Invalid usage: /username <username>");
            return;
        }
    };

    let json_data = serde_json::json!({"data-type": "username", "username": username});
    send(stream, &json_data);
}

fn send_message(stream: &TcpStream, message: &str) {
    let json_data = serde_json::json!({"data-type": "message", "content": message});
    send(stream, &json_data);
}

fn process_input(stream: TcpStream, listening: Arc<AtomicBool>) {
    loop {
        let mut raw_input = String::new();
        
        // print!("> ");
        // io::stdout().flush().unwrap();

        io::stdin().read_line(&mut raw_input).unwrap();

        let message_input = raw_input.trim();
        if message_input.is_empty() {
            continue;
        }

        if message_input.starts_with("/username") {
            send_username(&stream, message_input);
            continue;
        } else if message_input.eq_ignore_ascii_case("/quit") {
            println!("Quitting...");
            listening.store(false, Ordering::SeqCst);
            break;
        } else if message_input.starts_with("/") {
            println!("Unknown command, try either /username <username> or /quit");
        } else {
            send_message(&stream, message_input);
        }
    }
}

fn process_data(json_data: &serde_json::Value) {
    match json_data["data-type"].as_str() {
        Some("username") => {
            println!("* Your username has been set to: {}", json_data["username"].as_str().unwrap());
        }
        Some("message") => {
            println!("{}: {}", json_data["username"].as_str().unwrap(), json_data["content"].as_str().unwrap());
        }
        _ => {
            eprintln!("Received unknown data type: {json_data}");
        }
    }
}

fn listen(mut stream: TcpStream, listening: Arc<AtomicBool>) {
    println!("Listening to the server...");

    let mut buffer = [0; BUFFER_SIZE];

    stream.set_nonblocking(true).unwrap();

    while listening.load(Ordering::SeqCst) {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("No data received - likely server has closed the connection");
                break;
            }
            Ok(n) => {
                let received_data = String::from_utf8_lossy(&buffer[..n]).to_string();
                if let Ok(data) = serde_json::from_str::<Value>(&received_data) {
                    process_data(&data);
                } else {
                    eprintln!("Received malformed data from the server: {received_data}");
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
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
    let stream_clone = stream.try_clone().unwrap();

    let listening = Arc::new(AtomicBool::new(true));
    let listening_clone = listening.clone();

    let listen_thread = thread::spawn(move || listen(stream_clone, listening_clone));

    process_input(stream, listening.clone());

    listen_thread.join().unwrap();
}
