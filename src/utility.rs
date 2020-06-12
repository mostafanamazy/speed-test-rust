use chrono::Utc;
fn humanable_str(value: u64) -> String {
    let res = match value {
        v if v >= 1073741824 => format!("{} G", value >> 30),
        v if v >= 1048576 => format!("{} M", value >> 20),
        v if v >= 1024 => format!("{} K", value >> 10),
        _ => format!("{} ", value),
    };
    res
}

#[test]
fn test_humanable_str() {
    assert_eq!(humanable_str(20), "20 ");
    assert_eq!(humanable_str(1023), "1023 ");
    assert_eq!(humanable_str(1024), "1 K");
    assert_eq!(humanable_str(2048), "2 K");
    assert_eq!(humanable_str(1048575), "1023 K");
    assert_eq!(humanable_str(1048576), "1 M");
    assert_eq!(humanable_str(2097152), "2 M");
    assert_eq!(humanable_str(1073741823), "1023 M");
    assert_eq!(humanable_str(1073741824), "1 G");
    assert_eq!(humanable_str(2147483648), "2 G");
}

pub fn speed_string(bytes: usize, times: u64) -> String {
    let speed: u64 = bytes as u64 / times;
    assert!(times != 0);
    format!(
        "{}B/s (for {} second{})",
        humanable_str(speed),
        times,
        match times {
            1 => "",
            _ => "s",
        }
    )
}

pub fn current_time() -> String {
    format!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S"))
}
