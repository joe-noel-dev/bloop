#!/usr/bin/env bash

set -e
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly STATIC_CONTENT_DESTINATION=/var/www/html
readonly STATIC_CONTENT_SOURCE=frontend/dist/*
readonly BLOOP_CORE_BINARY=target/release/bloop-core
readonly BLOOP_CORE_DESTINATION=/usr/local/bin/bloop-core

install_core () {
    cargo build --release
    sudo rm -rf ${BLOOP_CORE_DESTINATION}
    sudo cp ${BLOOP_CORE_BINARY} ${BLOOP_CORE_DESTINATION}
}

install_frontend () {
    yarn install
    yarn run build
    sudo rm -rf ${STATIC_CONTENT_DESTINATION}
    sudo mkdir ${STATIC_CONTENT_DESTINATION}
    sudo cp -r ${STATIC_CONTENT_SOURCE} ${STATIC_CONTENT_DESTINATION}
}

install_services () {
    sudo systemctl stop bloop
    sudo systemctl stop jackd

    sudo cp scripts/bloop.service /etc/systemd/system
    sudo cp scripts/jackd.service /etc/systemd/system

    sudo systemctl daemon-reload

    sudo systemctl enable jackd
    sudo systemctl enable bloop

    sudo systemctl start jackd
    sudo systemctl start bloop
}

install_core
install_frontend
install_services