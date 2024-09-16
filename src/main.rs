use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex as TokioMutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Custom Redis Server running on port 8000");
    
    let store = Arc::new(TokioMutex::new(HashMap::new()));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let store = Arc::clone(&store);

        tokio::spawn(async move {
            let mut buffer = [0; 512];

            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return, // Connection closed
                    Ok(n) => n,
                    Err(_) => {
                        println!("Failed to read from socket");
                        return;
                    }
                };

                let command = String::from_utf8_lossy(&buffer[..n]);
                let parts: Vec<&str> = command.trim().split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                let response = match parts[0].to_lowercase().as_str() {
                    "set" if parts.len() == 3 => {
                        let key = parts[1].to_string();
                        let value = parts[2].to_string();
                        let mut store = store.lock().await;
                        store.insert(key, value);
                        "+OK\r\n".to_string()
                    }
                    "get" if parts.len() == 2 => {
                        let key = parts[1].to_string();
                        let store = store.lock().await;
                        if let Some(value) = store.get(&key) {
                            format!("${}\r\n{}\r\n", value.len(), value)
                        } else {
                            "$-1\r\n".to_string()
                        }
                    }
                    _ => "-ERR unknown command\r\n".to_string(),
                };

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    println!("Failed to write to socket: {}", e);
                    return;
                }
            }
        });
    }
}
