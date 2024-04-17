FROM rust:alpine AS builder

RUN apk add --no-cache \
    openssl-dev \
    musl-dev \
    gcc \
    openssl \
    openssl-dev \
    openssl-libs-static \
    ca-certificates \
    cargo

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --bin uwrss

FROM alpine:latest

RUN apk add --no-cache \
    openssl \
    ca-certificates

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/uwrss .

CMD ["./uwrss"]

ENV DOCKER_OPTS="--dns 8.8.8.8"
