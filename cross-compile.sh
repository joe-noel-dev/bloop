#!/usr/bin/env bash

set -ex

readonly CONTAINER_TAG=cross/bloop_aarch64:v1

docker build --tag ${CONTAINER_TAG} .

docker run \
--rm \
--volume .:/usr/src/bloop \
--workdir /usr/src/bloop \
--name bloop_cross \
${CONTAINER_TAG} \
cargo build --release --target aarch64-unknown-linux-gnu
