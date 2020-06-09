# speed-test-service
client and server for speed test

## Build
cargo build --release

## Run client 
./target/release/client `<path-to-config>`   
OR  
docker run --name client client:latest -v "$(pwd)"/client.toml:/etc/config.toml  
Example:   
./build/client config/client.ini

### output
  [Upload] `x` MB/s (for t seconds)   
  [Download] `x` MB/s (for t seconds)   

## Run server
./target/release/server `<path-to-config>`   
OR  
docker run --name server server:latest -v "$(pwd)"/server.toml:/etc/config.toml  
Example:   
./build/server config/server.ini
  
### output
  [connect-time] remote-address   
  Example:   
  [2020-01-20 12:32:48] 10.24.52.80:8000
