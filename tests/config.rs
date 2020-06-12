extern crate speed_test_rust;
use speed_test_rust::config;

#[test]
fn test_read_client() {
    let conf = config::read_client(Some("config/testfiles/1c.ini"));
    assert_eq!(conf.address, "127.0.0.1:55");
    assert_eq!(conf.timeout, 10);
    assert_eq!(conf.down_time, 30);
    assert_eq!(conf.up_time, 30);

    let conf = config::read_client(Some("config/testfiles/2c.ini"));
    assert_eq!(conf.address, "192.168.0.1:80");
    assert_eq!(conf.timeout, 10);
    assert_eq!(conf.down_time, 10);
    assert_eq!(conf.up_time, 10);
}

#[test]
fn test_read_server() {
    let conf = config::read_server(Some("config/testfiles/1s.ini"));
    assert_eq!(conf, "127.0.0.1:800");
}
