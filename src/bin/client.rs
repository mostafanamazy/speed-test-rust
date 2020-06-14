use std::error::Error;
use std::time::{Duration, Instant};
extern crate speed_test_rust;
use speed_test_rust::client::protocol::Protocol;
use speed_test_rust::client::streamer::Streamer;
use speed_test_rust::*;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = config::read_client(None);
    let mut stream = client::streamer::TcpStreamer::create(conf.address)?;
    stream.set_timeout(conf.timeout)?;
    let start_up = Instant::now();
    let mut total = 0;
    while start_up.elapsed() < Duration::new(conf.up_time, 0) {
        total += stream.upload()?;
    }
    println!(
        "[Upload] {}",
        utility::speed_string(total, start_up.elapsed().as_secs())
    );
    stream.upload_done()?;
    let start_down = Instant::now();
    total = 0;
    while start_down.elapsed() < Duration::new(conf.down_time, 0) {
        total += stream.download()?;
    }
    println!(
        "[Download] {}",
        utility::speed_string(total, start_down.elapsed().as_secs())
    );
    Ok(())
}
