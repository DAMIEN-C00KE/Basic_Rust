use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Could not bind");

    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed");
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream) {
    let response = b"Hello from server!";
    stream.write(response).expect("Failed to write response.");
}
