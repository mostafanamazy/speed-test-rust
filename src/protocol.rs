use crate::globals;
use async_trait::async_trait;
use std::error::Error;
use std::marker::Unpin;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::prelude::*;

async fn transfer<W>(socket: &mut W)
where
    W: AsyncWrite + AsyncRead + Unpin,
{
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
}
pub struct TcpProtocol {
    listener: TcpListener,
}
#[async_trait]
pub trait Protocol {
    async fn new(addr: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    async fn listen(&mut self) -> Result<SocketAddr, Box<dyn Error>>;
}
#[async_trait]
impl Protocol for TcpProtocol {
    async fn new(addr: String) -> Result<TcpProtocol, Box<dyn Error>> {
        Ok(TcpProtocol {
            listener: TcpListener::bind(addr).await?,
        })
    }
    async fn listen(&mut self) -> Result<SocketAddr, Box<dyn Error>> {
        let (mut stream, addr) = self.listener.accept().await?;
        tokio::spawn(async move { transfer(&mut stream).await });
        Ok(addr)
    }
}
