use ini::Ini;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn to_sec(st: &str) -> u64 {
    let (mut coef, ignores) = match st.chars().last() {
        Some(c) if c == 's' => (1, 1),
        Some(c) if c == 'm' => (60, 1),
        Some(c) if c == 'h' => (3600, 1),
        _ => (1, 0),
    };
    let num = match u64::from_str(&st[0..st.len() - ignores]) {
        Ok(n) if n > 0 => n,
        _ => {
            coef = 1;
            10
        }
    };
    num * coef
}

#[test]
fn test_to_sec() {
    assert_eq!(to_sec(&"20s"), 20);
    assert_eq!(to_sec(&"0"), 10);
    assert_eq!(to_sec(&"100"), 100);
    assert_eq!(to_sec(&"0h"), 10);
    assert_eq!(to_sec(&"10m"), 600);
}

struct Config {
    address: String,
    timeout: u64,
    down_time: u64,
    up_time: u64,
}
fn read_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut ret = Config {
        address: "127.0.0.1:5555".to_string(),
        timeout: 10,
        down_time: 10,
        up_time: 10,
    };
    if args.len() < 2 {
        eprintln!("please select config file. DEFAULT SELECTED");
    } else {
        let arg = &args[1];
        let i = Ini::load_from_file(arg).unwrap();
        for (sec, prop) in i.iter() {
            if sec == Some("server") {
                for (k, v) in prop.iter() {
                    match k {
                        "address" => ret.address = v.to_string(),
                        "timeout" => ret.timeout = to_sec(&v),
                        _ => (),
                    };
                }
            } else if sec == Some("test") {
                for (k, v) in prop.iter() {
                    match k {
                        "download_time" => ret.down_time = to_sec(&v),
                        "upload_time" => ret.up_time = to_sec(&v),
                        _ => (),
                    };
                }
            }
        }
    }
    ret
}

fn humanable_string(bytes: usize, times: u64) -> String {
    let speed: u64 = bytes as u64 / times;
    assert!(times != 0);
    let res = match speed {
        s if s >= 1073741824 => format!("{} GB/s (for {} seconds)", speed >> 30, times),
        s if s >= 1048576 => format!("{} MB/s (for {} seconds)", speed >> 20, times),
        s if s >= 1024 => format!("{} KB/s (for {} seconds)", speed >> 10, times),
        _ => format!("{} B/s (for {} seconds)", speed, times),
    };
    res
}
fn main() {
    let conf = read_config();
    let mut stream = TcpStream::connect(conf.address).unwrap();
    let mut buf = [30; 1024];
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
    println!("[Upload] {}", humanable_string(total, conf.up_time));
    stream.write(&msg).expect("could not write msg.");
    let start_down = Instant::now();
    total = 0;
    while start_down.elapsed() < Duration::new(conf.down_time, 0) {
        total += stream.read(&mut buf).expect("could not read.");
    }
    println!("[Download] {}", humanable_string(total, conf.down_time));
}
