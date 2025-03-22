#!/usr/bin/env bash

set -ex

API_DIR=./api
OUT_DIR="${API_DIR}/out"
RUST_OUT_DIR="${OUT_DIR}/rust"
CPP_OUT_DIR="${OUT_DIR}/cpp"

mkdir -p "${RUST_OUT_DIR}"
protoc --rust_out="${RUST_OUT_DIR}" --proto_path="${API_DIR}" --rust_opt=experimental-codegen=enabled "${API_DIR}"/*.proto

mkdir -p "${CPP_OUT_DIR}"
protoc --cpp_out="${CPP_OUT_DIR}" --proto_path="${API_DIR}" "${API_DIR}"/*.proto