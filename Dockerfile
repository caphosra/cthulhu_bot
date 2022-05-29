FROM rust:1.61.0-slim

WORKDIR /app

COPY . /app

RUN cd /app \
    && apt update \
    && apt upgrade -y \
    && apt install libssl-dev -y \
    && cargo build --release \
    && cp /app/target/release/cthulhu_bot /app

CMD ./cthulhu_bot
