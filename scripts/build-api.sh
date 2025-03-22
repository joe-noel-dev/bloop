#!/usr/bin/env bash

set -ex

API_DIR=./api
OUT_DIR="${API_DIR}/out"
SWIFT_OUT_DIR="${OUT_DIR}/swift"
PROTO_SOURCE="${API_DIR}/bloop.proto"

mkdir -p "${SWIFT_OUT_DIR}"
protoc --proto_path="${API_DIR}" --swift_out="${SWIFT_OUT_DIR}" "${PROTO_SOURCE}"