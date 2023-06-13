#!/usr/bin/env bash

set -ex

readonly CONTAINER_TAG=cross/bloop_arm64v8:v1
readonly CONTAINER_SRC_DIR=/usr/src/bloop

docker build --tag ${CONTAINER_TAG} .

docker run \
--rm \
--volume .:${CONTAINER_SRC_DIR} \
--workdir ${CONTAINER_SRC_DIR} \
--name bloop_cross \
${CONTAINER_TAG} \
cargo build --release
