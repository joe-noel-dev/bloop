#!/usr/bin/env bash

set -ex

ARCH=${1:-arm64} # Options include: amd64, arm64
CONTAINER_TAG="cross/bloop_${ARCH}:v1"
CONTAINER_SRC_DIR=/usr/src/bloop
DOCKERFILE="Dockerfile"
PLATFORM="linux/${ARCH}"

# Use the official multi-arch rust image for all architectures
BASE_IMAGE="rust:1.88.0"

docker build --file ${DOCKERFILE} --build-arg BASE_IMAGE=${BASE_IMAGE} --tag ${CONTAINER_TAG} --platform ${PLATFORM} .

docker run \
    --rm \
    --platform ${PLATFORM} \
    --volume .:${CONTAINER_SRC_DIR} \
    --workdir ${CONTAINER_SRC_DIR}/core \
    --name bloop_cross_${ARCH} \
    ${CONTAINER_TAG} \
    cargo build --release
