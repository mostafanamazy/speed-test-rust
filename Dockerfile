FROM rust:1.41

WORKDIR /usr/src/speed-test-rust
COPY . .

RUN cargo test
RUN cargo install --path .

CMD ["server"]
CMD ["client"]

