extern crate speed_test_rust;
use speed_test_rust::utility;

#[test]
fn test_speed_string() {
    assert_eq!(utility::speed_string(20, 2), "10 B/s (for 2 seconds)");
    assert_eq!(utility::speed_string(1023, 1), "1023 B/s (for 1 second)");
    assert_eq!(utility::speed_string(2048, 2), "1 KB/s (for 2 seconds)");
    assert_eq!(utility::speed_string(40366, 4), "9 KB/s (for 4 seconds)");
    assert_eq!(utility::speed_string(32505856, 6), "5 MB/s (for 6 seconds)");
}
