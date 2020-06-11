use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{Duration, Instant};
extern crate speed_test_rust;
use speed_test_rust::*;

fn main() {
    let conf = config::read_client(None);
    let mut stream = TcpStream::connect(conf.address).unwrap();
    let mut buf = [30; globals::BUFF_MAX_SIZE];
    let msg = [0; 4];
    stream
        .set_read_timeout(Some(Duration::new(conf.timeout, 0)))
        .expect("could not read time out");
    stream
        .set_write_timeout(Some(Duration::new(conf.timeout, 0)))
        .expect("could not write timeout");
    let start_up = Instant::now();
    let mut total = 0;
    while start_up.elapsed() < Duration::new(conf.up_time, 0) {
        total += stream.write(&buf).expect("could not write.");
    }
    println!("[Upload] {}", utility::speed_string(total, conf.up_time));
    stream.write(&msg).expect("could not write msg.");
    let start_down = Instant::now();
    total = 0;
    while start_down.elapsed() < Duration::new(conf.down_time, 0) {
        total += stream.read(&mut buf).expect("could not read.");
    }
    println!(
        "[Download] {}",
        utility::speed_string(total, conf.down_time)
    );
}
