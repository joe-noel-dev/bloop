#!/usr/bin/env bash

set -e

if [ -z "$BUILD_NUMBER" ]; then
  echo "Error: BUILD_NUMBER environment variable is not set" >&2
  exit 1
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
 
cd ${PROJECT_ROOT}
./scripts/build-api-ios.sh
./scripts/build-xcframework.sh

cd ${PROJECT_ROOT}/ios
bundle install
bundle exec fastlane beta

