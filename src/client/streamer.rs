use crate::client::protocol::{Protocol, TcpProtocol};
use std::error::Error;

pub struct TcpStreamer;
pub trait Streamer {
    type Protocol;
    fn create(adrr: String) -> Result<Self::Protocol, Box<dyn Error>>;
}
impl Streamer for TcpStreamer {
    type Protocol = TcpProtocol;
    fn create(addr: String) -> Result<TcpProtocol, Box<dyn Error>> {
        Ok(TcpProtocol::new(addr)?)
    }
}
