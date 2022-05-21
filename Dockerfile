FROM rust:1.61.0-slim

WORKDIR /app

COPY . /app

CMD cargo run --release
