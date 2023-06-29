#!/usr/bin/env bash

set -ex

swift-format lint -r source tests ui-tests
