#!/bin/bash
./target/release/server config/server.ini &
sleep 1

for i in {1..6}
do
	./target/release/client config/client.ini &
done

