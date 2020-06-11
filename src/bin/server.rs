use chrono::Utc;
use tokio::net::TcpListener;
use tokio::prelude::*;
extern crate speed_test_rust;
use speed_test_rust::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind(config::read_server(None)).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), addr);

        tokio::spawn(async move {
            let mut buf = [30; globals::BUFF_MAX_SIZE];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if buf[n - 1] == 0 {
                    loop {
                        if let Err(_) = socket.write_all(&buf).await {
                            break;
                        }
                    }
                }
            }
        });
    }
}
