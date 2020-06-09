use ini::Ini;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::str::FromStr;

fn to_sec(st: &String) -> u64 {
    let num = match u64::from_str(st) {
        Ok(n) => n,
        Err(_) => 10,
    };
    num
}

#[test]
fn test_to_sec() {
    assert_eq!(to_sec(&"20".to_string()), 20);
}

fn read_config() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("please select config file.");
    }
    else {
        let arg = &args[1];
        let i = Ini::load_from_file(arg).unwrap();
        for (sec, prop) in i.iter() {
            println!("Section: {:?}", sec);
            for (k, v) in prop.iter() {
                println!("{}:{}", k, v);
            }
        }
    }
}

fn humanable_string(bytes: usize, times: u64) -> String {
    assert!(times != 0);
    let res = match bytes {
        b if b >= 1073741824 => format!("{} GB/s (for {} seconds)", bytes >> 30, times),
        b if b >= 1048576 => format!("{} MB/s (for {} seconds)", bytes >> 20, times),
        b if b >= 1024 => format!("{} KB/s (for {} seconds)", bytes >> 10, times),
        _ => format!("{} B/s (for {} seconds)", bytes, times),
    };
    res
}
fn main() {
    read_config();
    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();
    let mut buf = [30; 1024];
    let msg = [0; 4];
    let up_time = 5;
    let down_time = 5;
    let time_out = 10;
    stream.set_read_timeout(Some(Duration::new(time_out, 0))).expect("could not read time out");
    stream.set_write_timeout(Some(Duration::new(time_out, 0))).expect("could not write timeout");
    let start_up = Instant::now();
    let mut total = 0;
    while start_up.elapsed() < Duration::new(up_time, 0) {
        total += stream.write(&buf).expect("could not write.");
    }
    println!("[Upload] {}", humanable_string(total, up_time));
    stream.write(&msg).expect("could not write msg.");
    let start_down = Instant::now();
    total = 0;
    while start_down.elapsed() < Duration::new(down_time, 0) {
        total += stream.read(&mut buf).expect("could not read.");
    }
    println!("[Download] {}", humanable_string(total, down_time));

}

