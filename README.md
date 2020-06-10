# speed-test-service
client and server for speed test

## Build
cargo build --release  
OR  
docker build -t speed-test-rust .

## Run client 
./target/release/client `<path-to-config>`   
OR  
docker run -it --rm --name client speed-test-rust client /usr/src/speed-test-rust/config/client.ini
Example:   
./build/client config/client.ini

### output
  [Upload] `x` MB/s (for t seconds)   
  [Download] `x` MB/s (for t seconds)   

## Run server
./target/release/server `<path-to-config>`   
OR  
docker run -it --rm --name server speed-test-rust server /usr/src/speed-test-rust/config/server.ini 
Example:   
./build/server config/server.ini
  
### output
  [connect-time] remote-address   
  Example:   
  [2020-01-20 12:32:48] 10.24.52.80:8000