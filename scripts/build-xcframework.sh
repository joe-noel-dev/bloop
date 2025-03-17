#!/usr/bin/env bash

set -e

LIB_NAME="bloop"

rm -rf ./target/universal-ios
mkdir -p ./target/universal-ios

cargo build --no-default-features --target aarch64-apple-ios
cargo build --release --no-default-features --target aarch64-apple-ios

# lipo -create \
#     ./target/aarch64-apple-ios/debug/lib${LIB_NAME}.a \
#     ./target/aarch64-apple-ios/release/lib${LIB_NAME}.a \
#     -output ./target/universal-ios/lib${LIB_NAME}.a 

lipo -create \
    ./target/aarch64-apple-ios/release/lib${LIB_NAME}.a \
    -output ./target/universal-ios/lib${LIB_NAME}.a 

xcodebuild -create-xcframework \
    -library ./target/universal-ios/lib${LIB_NAME}.a \
    -headers ./target/include \
    -output ./target/universal-ios/${LIB_NAME}.xcframework