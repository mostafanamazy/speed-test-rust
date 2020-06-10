use chrono::Utc;
use ini::Ini;
use std::env;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn server_address() -> String {
    let args: Vec<String> = env::args().collect();
    let res: String = "".to_string();
    if args.len() < 2 {
        eprintln!("please select config file.");
        return res;
    } else {
        let arg = &args[1];
        let i = Ini::load_from_file(arg).unwrap();
        for (sec, prop) in i.iter() {
            if sec == Some("server") {
                for (k, v) in prop.iter() {
                    if k == "address" {
                        return v.to_string();
                    }
                }
            }
        }
    }
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind(server_address()).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), addr);

        tokio::spawn(async move {
            let mut buf = [30; 1024];

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
