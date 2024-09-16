use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
            stream.write(b"Response from server\n").unwrap();
        }
        Err(e) => {
            println!("Failed to read from client: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    // Bind the server to localhost:8000
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening on port 8000");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
