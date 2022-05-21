FROM rust:1.61.0-slim

WORKDIR /app

COPY . /app

RUN cd /app \
    && cargo build --release \
    && cp /app/target/release/ctulhu_bot /app

CMD ./ctulhu_bot
