#!/usr/bin/env bash

set -e
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly STATIC_CONTENT_DESTINATION=/var/www/html
readonly STATIC_CONTENT_SOURCE=frontend/dist/*

install_core () {
    cargo build --release
}

install_frontend () {
    yarn install
    yarn run build
    sudo rm -rf ${STATIC_CONTENT_DESTINATION}
    sudo mkdir ${STATIC_CONTENT_DESTINATION}
    sudo cp -r ${STATIC_CONTENT_SOURCE} ${STATIC_CONTENT_DESTINATION}
}

install_core
install_frontend
