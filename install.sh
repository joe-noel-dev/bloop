#!/usr/bin/env bash

set -e
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly BLOOP_CORE_BINARY=target/release/bloop-core
readonly BLOOP_CORE_DESTINATION=/usr/local/bin/bloop-core

install_core () {
    cargo build --release
    sudo rm -rf ${BLOOP_CORE_DESTINATION}
    sudo cp ${BLOOP_CORE_BINARY} ${BLOOP_CORE_DESTINATION}
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
install_services