FROM rust:latest as builder

RUN USER=root cargo new --lib build-deps
WORKDIR /build-deps

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# removing tests config from Cargo.toml
RUN sed '/\[\[test\]\]/,$d' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

RUN cargo build --lib
RUN cargo build --tests

RUN rm -rf src
RUN rm Cargo.toml

ADD . ./

CMD ["cargo test --test xchange"]
