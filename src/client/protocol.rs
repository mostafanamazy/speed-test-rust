use crate::globals;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

pub struct TcpProtocol {
    handler: TcpStream,
}

pub trait Protocol {
    fn new(addr: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn set_timeout(&mut self, timeout: u64) -> Result<(), Box<dyn Error>>;
    fn upload(&mut self) -> Result<usize, Box<dyn Error>>;
    fn upload_done(&mut self) -> Result<(), Box<dyn Error>>;
    fn download(&mut self) -> Result<usize, Box<dyn Error>>;
}

impl Protocol for TcpProtocol {
    fn new(addr: String) -> Result<TcpProtocol, Box<dyn Error>> {
        Ok(TcpProtocol {
            handler: TcpStream::connect(addr)?,
        })
    }
    fn set_timeout(&mut self, timeout: u64) -> Result<(), Box<dyn Error>> {
        self.handler
            .set_read_timeout(Some(Duration::new(timeout, 0)))?;
        self.handler
            .set_write_timeout(Some(Duration::new(timeout, 0)))?;
        Ok(())
    }
    fn upload(&mut self) -> Result<usize, Box<dyn Error>> {
        let buf = [30; globals::BUFF_MAX_SIZE];
        Ok(self.handler.write(&buf)?)
    }
    fn upload_done(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = [0; 4];
        self.handler.write(&msg)?;
        Ok(())
    }
    fn download(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut buf = [30; globals::BUFF_MAX_SIZE];
        Ok(self.handler.read(&mut buf)?)
    }
}
