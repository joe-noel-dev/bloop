#!/usr/bin/env bash

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
 
cd ${PROJECT_ROOT}
./scripts/build-api-ios.sh
./scripts/build-xcframework.sh

cd ${PROJECT_ROOT}/ios
bundle install
bundle exec fastlane dev

