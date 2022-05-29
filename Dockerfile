FROM rust:1.61.0-slim

WORKDIR /app

COPY . /app

RUN cd /app \
    && apt update \
    && apt upgrade -y \
    && apt install pkg-config libssl-dev -y \
    && export PKG_CONFIG_ALLOW_CROSS=1 \
    && cargo build --release \
    && cp /app/target/release/cthulhu_bot /app

CMD ./cthulhu_bot
