use ini::Ini;
use std::env;
use std::str::FromStr;

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
    assert_eq!(to_sec(&"100H"), 10);
    assert_eq!(to_sec(&"2h"), 7200);
    assert_eq!(to_sec(&"10m"), 600);
}

pub struct ClientConfig {
    pub address: String,
    pub timeout: u64,
    pub down_time: u64,
    pub up_time: u64,
}

fn arg_file(input: Option<String>) -> String {
    if let Some(x) = input {
        return x;
    }

    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return "".to_string();
    }
    let ret = &mut args[1];
    ret.to_string()
}

pub fn read_client(input: Option<String>) -> ClientConfig {
    let i = Ini::load_from_file(arg_file(input)).unwrap();
    let server = i.section(Some("server")).unwrap();
    let test = i.section(Some("test")).unwrap();
    ClientConfig {
        address: server.get("address").unwrap().to_string(),
        timeout: to_sec(server.get("timeout").unwrap()),
        down_time: to_sec(test.get("download_time").unwrap()),
        up_time: to_sec(test.get("upload_time").unwrap()),
    }
}

pub fn read_server(input: Option<String>) -> String {
    let i = Ini::load_from_file(arg_file(input)).unwrap();
    let server = i.section(Some("server")).unwrap();
    let ip = server.get("ip").unwrap();
    let port = server.get("port").unwrap();
    format!("{}:{}", ip, port)
}
