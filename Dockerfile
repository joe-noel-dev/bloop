ARG BASE_IMAGE=rust:1.71.0
FROM ${BASE_IMAGE}

ENV DEBIAN_FRONTEND=noninteractive

# Only run this for ARM64
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "arm64" ]; then dpkg --add-architecture arm64; fi

RUN apt-get update && \
    apt-get install -y \
        libasound2-dev \
        libudev-dev \
        jackd2 \
        libjack-jackd2-dev \
        libgl1-mesa-dev \
        libegl1-mesa-dev \
        libgles2-mesa-dev \
        libx11-dev \
        protobuf-compiler \
        libssl-dev \
        pkg-config \
        build-essential \
        && rm -rf /var/lib/apt/lists/*

RUN rustup default nightly
