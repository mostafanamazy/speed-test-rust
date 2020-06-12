use crate::protocol::{Protocol, TcpProtocol};
use async_trait::async_trait;
use std::error::Error;

pub struct TcpStreamer;
#[async_trait]
pub trait Streamer {
    type Protocol;
    async fn create(adrr: String) -> Result<Self::Protocol, Box<dyn Error>>;
}
#[async_trait]
impl Streamer for TcpStreamer {
    type Protocol = TcpProtocol;
    async fn create(addr: String) -> Result<TcpProtocol, Box<dyn Error>> {
        Ok(TcpProtocol::new(addr).await?)
    }
}
