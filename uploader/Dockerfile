FROM rustlang/rust:nightly AS builder

RUN USER=root cargo new --bin tiny_uploader
WORKDIR /tiny_uploader

# copy over your manifests
#COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/tiny_uploader*
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder  /tiny_uploader/target/release/tiny_uploader /tiny_uploader/tiny_uploader
COPY ./migrations /tiny_uploader/migrations/
WORKDIR /tiny_uploader/
EXPOSE 8088


CMD ["/tiny_uploader/tiny_uploader"]
