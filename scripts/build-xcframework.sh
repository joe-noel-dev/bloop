#!/usr/bin/env bash

set -e

SOURCE_LIB_NAME="bloop"
DEST_LIB_NAME="bloop"
DEPLOYMENT_TARGET="15.0"
CORE_DIR="./core"

rm -rf ./target/universal-ios
mkdir -p ./target/universal-ios

build_target() {
  local build_type=$1 # "release" or "debug"
  local target=$2     # Rust target triple
  local sdk=$3        # iphoneos or iphonesimulator
  local sdk_version_flag=$4
  local extra_flags=$5

  export SDKROOT=$(xcrun --sdk ${sdk} --show-sdk-path)
  export IPHONEOS_DEPLOYMENT_TARGET="${DEPLOYMENT_TARGET}"
  export RUSTFLAGS="-C link-arg=${sdk_version_flag}=${DEPLOYMENT_TARGET} ${extra_flags}"

  if [ "$build_type" = "release" ]; then
    (cd ${CORE_DIR} && cargo build --release --no-default-features --features midi --target ${target})
  else
    (cd ${CORE_DIR} && cargo build --no-default-features --features midi --target ${target})
  fi
}

# Release builds
build_target release aarch64-apple-ios iphoneos -mios-version-min
build_target release aarch64-apple-ios-sim iphonesimulator -mios-simulator-version-min

# Debug builds
build_target debug aarch64-apple-ios iphoneos -mios-version-min
build_target debug aarch64-apple-ios-sim iphonesimulator -mios-simulator-version-min

# Generate XCFrameworks
xcodebuild -create-xcframework \
  -library ./core/target/aarch64-apple-ios/release/lib${SOURCE_LIB_NAME}.a \
  -headers ./core/target/include \
  -library ./core/target/aarch64-apple-ios-sim/release/lib${SOURCE_LIB_NAME}.a \
  -headers ./core/target/include \
  -output ./target/universal-ios/${DEST_LIB_NAME}.xcframework

xcodebuild -create-xcframework \
  -library ./core/target/aarch64-apple-ios/debug/lib${SOURCE_LIB_NAME}.a \
  -headers ./core/target/include \
  -library ./core/target/aarch64-apple-ios-sim/debug/lib${SOURCE_LIB_NAME}.a \
  -headers ./core/target/include \
  -output ./target/universal-ios/${DEST_LIB_NAME}_Debug.xcframework
