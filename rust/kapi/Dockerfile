FROM rust:1.55 AS rbuild

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /src/kapi
COPY . .

# build deps
RUN cargo build

