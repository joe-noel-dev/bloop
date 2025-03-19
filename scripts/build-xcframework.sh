#!/usr/bin/env bash

set -e

LIB_NAME="bloop"

rm -rf ./target/universal-ios
mkdir -p ./target/universal-ios

cargo build --release --no-default-features --target aarch64-apple-ios
cargo build --release --no-default-features --target aarch64-apple-ios-sim
cargo build --no-default-features --target aarch64-apple-ios
cargo build --no-default-features --target aarch64-apple-ios-sim

xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/release/lib${LIB_NAME}.a \
    -headers ./target/include \
    -library ./target/aarch64-apple-ios-sim/release/lib${LIB_NAME}.a \
    -headers ./target/include \
    -output ./target/universal-ios/${LIB_NAME}.xcframework

xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/debug/lib${LIB_NAME}.a \
    -headers ./target/include \
    -library ./target/aarch64-apple-ios-sim/debug/lib${LIB_NAME}.a \
    -headers ./target/include \
    -output ./target/universal-ios/${LIB_NAME}_Debug.xcframework