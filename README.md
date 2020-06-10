# speed-test-service
client and server for speed test

## Build
$ cargo build --release  
OR  
$ docker build -t speed-test-rust .

## Run client 
$ ./target/release/client `<path-to-config>`  
OR  
$ docker run --name client speed-test-rust client /usr/src/speed-test-rust/config/client.ini  
Example:  
$ ./target/release/client config/client.ini  

### output
  [Upload] `x` MB/s (for t seconds)   
  [Download] `x` MB/s (for t seconds)   

## Run server
$ ./target/release/server `<path-to-config>`   
OR  
$ docker run --expose `port` -p `port`:`port` --name server speed-test-rust server /usr/src/speed-test-rust/config/server.ini  
Examples:  
$ ./target/release/server config/server.ini  
$ docker run --expose 5555 -p 5555:5555 --name server speed-test-rust server /usr/src/speed-test-rust/config/server.ini  
  
### output
  [connect-time] remote-address   
  Example:   
  [2020-01-20 12:32:48] 10.24.52.80:8000