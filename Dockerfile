FROM rust:1.69

ENV PKG_CONFIG_ALLOW_CROSS 1
ENV PKG_CONFIG_PATH /usr/lib/aarch64-linux-gnu/pkgconfig/
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER /usr/bin/aarch64-linux-gnu-gcc

RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install libasound2-dev:arm64 -y

RUN rustup default nightly
RUN rustup target add aarch64-unknown-linux-gnu
