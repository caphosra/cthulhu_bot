#
# Building step
#
FROM clux/muslrust:1.85.1-stable AS build

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . /app

RUN \
    cd /app; \
    cargo build --release;

#
# Deploy step
#
FROM alpine:3.21 AS deploy

WORKDIR /app

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/cthulhu_bot /app/cthulhu_bot
