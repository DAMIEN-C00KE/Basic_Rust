use std::io::prelude::*;
use std::net::TcpStream;

pub fn start() {
    let mut stream = TcpStream::connect("0.0.0.0:8080").expect("Could not connect to server");
    let mut buffer = [0; 128];

    stream.read(&mut buffer).expect("Failed to read from server");
    let message = String::from_utf8_lossy(&buffer[..]);
    println!("Received: {}", message);
}
