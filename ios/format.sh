#!/usr/bin/env bash

set -ex

swift-format format -i -r source tests ui-tests

