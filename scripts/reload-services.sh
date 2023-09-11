#!/usr/bin/env bash

# ssh joe@bloop.local 'bash -s' < ./scripts/reload-services.sh

set -ex

sudo systemctl stop bloop
sudo systemctl stop jackd

sudo systemctl daemon-reload

sudo systemctl enable jackd
sudo systemctl enable bloop

sudo systemctl start jackd
sudo systemctl start bloop