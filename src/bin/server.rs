use std::error::Error;
extern crate speed_test_rust;
use speed_test_rust::server::protocol::Protocol;
use speed_test_rust::server::streamer::Streamer;
use speed_test_rust::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = server::streamer::TcpStreamer::create(config::read_server(None)).await?;

    loop {
        let addr = listener.listen().await?;
        println!("[{}] {}", utility::current_time(), addr);
    }
}
