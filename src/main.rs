use std::net::TcpStream;
use std::io::Write;
use std::io;


fn main() {
    match TcpStream::connect("localhost:6969") {
        Ok(mut stream) => {
            println!("Successfully connected to server on port 6969");

            let mut message = String::new();
            loop {
                message.clear();
                io::stdin().read_line(&mut message).expect("Failed to read input");

                let trimmed_message = message.trim();
                if trimmed_message == "end" {
                    break;
                }

                stream.write(trimmed_message.as_bytes()).unwrap();
            }
            
        },
        Err(e) => {
            println!("Failed to connect: {e}")
        }
    }
    println!("Terminated!")
}
