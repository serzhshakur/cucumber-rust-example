FROM rust:1.60.0

RUN cargo new --lib cucumber-tests
WORKDIR /cucumber-tests

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# removing tests config from Cargo.toml
RUN sed '/\[\[test\]\]/,$d' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# caching dependencies
RUN cargo build --lib
RUN cargo build --tests

RUN rm -rf src
RUN rm Cargo.toml

ENV PROFILE=ci

ADD . ./
RUN chmod +x ./entrypoint.sh

CMD ["./entrypoint.sh"]
