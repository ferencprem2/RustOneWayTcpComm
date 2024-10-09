use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 { return; }
            stream.write(&buffer[0..size]).unwrap();
            let message = String::from_utf8_lossy(&buffer[..size]);
            println!("Client: {}", message);
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("localhost:6969").unwrap();
    println!("Server listening on port 6969");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}