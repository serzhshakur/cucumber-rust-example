FROM rust:latest

RUN cargo new --lib cucumber-tests
WORKDIR /cucumber-tests

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# removing tests config from Cargo.toml
RUN sed '/\[\[test\]\]/,$d' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

RUN cargo build --lib
RUN cargo build --tests

RUN rm -rf src
RUN rm Cargo.toml
ENV PROFILE=ci

ADD . ./

CMD ["cargo", "test", "--test", "xchange"]
