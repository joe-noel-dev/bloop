#!/usr/bin/env bash

set -euo pipefail

TARGET_TRIPLE="aarch64-apple-darwin"
APP_NAME="Bloop"
SIGNING_IDENTITY="${MACOS_CODESIGN_IDENTITY:--}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
CORE_DIR="${ROOT_DIR}/core"
WORK_DIR="${ROOT_DIR}/target/macos-app"
PACKAGE_DIR="${WORK_DIR}/package"
CARGO_PACKAGER_CACHE_DIR="${WORK_DIR}/cache"
ICON_SOURCE="${ROOT_DIR}/ios/source/app/Assets.xcassets/AppIcon.appiconset/App_store_1024_1x.png"
OPAQUE_ICON_SOURCE="${WORK_DIR}/${APP_NAME}-512x512@2x.png"
SWIFT_MODULE_CACHE="${WORK_DIR}/swift-module-cache"
PACKAGER_HOME="${WORK_DIR}/home"
INFO_PLIST_TEMPLATE="${CORE_DIR}/packaging/macos/Info.plist.template"
INFO_PLIST_PATH="${WORK_DIR}/Info.plist"
ENTITLEMENTS_PATH="${CORE_DIR}/packaging/macos/entitlements.plist"
FLATTEN_ICON_SCRIPT="${CORE_DIR}/packaging/macos/flatten-icon.swift"
PACKAGER_CONFIG="${CORE_DIR}/Packager.toml"

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "Error: macOS app bundles must be built on macOS."
    exit 1
fi

if [[ ! -f "${ICON_SOURCE}" ]]; then
    echo "Error: icon source not found at ${ICON_SOURCE}"
    exit 1
fi

mkdir -p "${WORK_DIR}" "${CARGO_PACKAGER_CACHE_DIR}" "${SWIFT_MODULE_CACHE}" "${PACKAGER_HOME}"
export XDG_CACHE_HOME="${CARGO_PACKAGER_CACHE_DIR}"

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' "${CORE_DIR}/Cargo.toml" | head -n 1)"
if [[ -z "${VERSION}" ]]; then
    echo "Error: unable to read core package version."
    exit 1
fi

sed "s/@VERSION@/${VERSION}/g" "${INFO_PLIST_TEMPLATE}" > "${INFO_PLIST_PATH}"

swift -module-cache-path "${SWIFT_MODULE_CACHE}" "${FLATTEN_ICON_SCRIPT}" "${ICON_SOURCE}" "${OPAQUE_ICON_SOURCE}"

if ! command -v cargo-packager >/dev/null 2>&1; then
    cargo install cargo-packager --locked
fi
PACKAGER_BIN="$(command -v cargo-packager)"

rustup target add "${TARGET_TRIPLE}"

(
    cd "${CORE_DIR}"
    cargo build --release --target "${TARGET_TRIPLE}"
    HOME="${PACKAGER_HOME}" "${PACKAGER_BIN}" --formats app --config "${PACKAGER_CONFIG}"
)

APP_PATH="$(find "${PACKAGE_DIR}" -maxdepth 4 -type d -name "${APP_NAME}.app" -print -quit)"
if [[ -z "${APP_PATH}" ]]; then
    echo "Error: ${APP_NAME}.app was not created under ${PACKAGE_DIR}"
    exit 1
fi

codesign --force --deep --entitlements "${ENTITLEMENTS_PATH}" --sign "${SIGNING_IDENTITY}" "${APP_PATH}"
codesign --verify --deep --strict --verbose=2 "${APP_PATH}"

echo "Built ${APP_PATH}"
