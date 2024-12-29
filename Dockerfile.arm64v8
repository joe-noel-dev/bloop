FROM arm64v8/rust:1.69

RUN apt-get update && \
    apt-get install libasound2-dev:arm64 libudev-dev:arm64 -y

RUN rustup default nightly
