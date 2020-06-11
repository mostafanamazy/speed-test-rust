#!/bin/bash
for i in {1..14}
do
	./target/release/client config/client.ini &
done

