#
# Building step
#
FROM clux/muslrust:1.85.1-stable AS build

ARG BOT_VER

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . /app

RUN \
    cd /app; \
    git clone https://github.com/caphosra/cthulhu_bot.git; \
    cd cthulhu_bot; \
    git checkout -b $BOT_VER; \
    cargo build --release;

#
# Deploy step
#
FROM alpine:3.21 AS deploy

WORKDIR /app

COPY --from=build /app/cthulhu_bot/target/x86_64-unknown-linux-musl/release/cthulhu_bot /app/cthulhu_bot
