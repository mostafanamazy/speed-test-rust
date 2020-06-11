extern crate speed_test_rust;
use speed_test_rust::config;

#[test]
fn test_read_client() {
    let conf = config::read_client(Some("config/testfile/1c.ini".to_string()));
    assert_eq!(conf.address, "127.0.0.1:55");
    assert_eq!(conf.timeout, 10);
    assert_eq!(conf.down_time, 30);
    assert_eq!(conf.up_time, 30);
}

#[test]
fn test_read_server() {
    let conf = config::read_server(Some("config/testfile/1s.ini".to_string()));
    assert_eq!(conf, "127.0.0.1:800");
}
