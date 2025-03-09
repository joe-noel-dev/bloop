#!/usr/bin/env bash

set -ex

ARCH=${1:-arm64v8} # Options include: amd64, arm64v8
CONTAINER_TAG="cross/bloop_${ARCH}:v1"
CONTAINER_SRC_DIR=/usr/src/bloop
DOCKERFILE="Dockerfile.${ARCH}"
PLATFORM="linux/${ARCH}"

docker build --file ${DOCKERFILE} --tag ${CONTAINER_TAG} --platform ${PLATFORM} . 

docker run \
--rm \
--platform ${PLATFORM} \
--volume .:${CONTAINER_SRC_DIR} \
--workdir ${CONTAINER_SRC_DIR} \
--name bloop_cross_${ARCH} \
${CONTAINER_TAG} \
cargo build --release
