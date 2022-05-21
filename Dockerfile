FROM rust:1.61.0-slim

WORKDIR /app

COPY . /app

RUN cd /app \
    && cargo build --release \
    && cp /app/target/release/cthulhu_bot /app

CMD ./cthulhu_bot
