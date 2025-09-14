#!/usr/bin/env bash

set -e


CARGO_TOML_PATH="../Cargo.toml"
VERSION_FILE_PATH="./source/app/Version.swift"

if [ ! -f "$CARGO_TOML_PATH" ]; then
    echo "Error: $CARGO_TOML_PATH does not exist."
    exit 1
fi

# '-m 1' 
#   first match
# '^version = ' 
#   line to match
# s/version = "\(.*\)"/\1/ 
#   extract version number
VERSION=$(grep -m 1 '^version = ' "${CARGO_TOML_PATH}" | sed 's/version = "\(.*\)"/\1/')

GIT_SHA=$(git rev-parse --short HEAD)

if [ -n "$(git status --porcelain)" ]; then
    GIT_SHA="${GIT_SHA}-modified"
fi

DISPLAY_VERSION="${VERSION} (${GIT_SHA})"

# Hack the project file. Other approaches didn't work:
# 
# Use PlistBuddy to update Info.plist. This didn't work as Xcode overwrites the
# versions using the MARKETING_VERSION and CURRENT_PROJECT_VERSION values in the
# project file
#
# Use agvtool. This didn't work as it didn't update the MARKETING_VERSION

XCODE_PROJECT="./Bloop.xcodeproj/project.pbxproj"
sed -i '' "s/MARKETING_VERSION = .*;/MARKETING_VERSION = \"${VERSION}\";/" "${XCODE_PROJECT}"
sed -i '' "s/CURRENT_PROJECT_VERSION = .*;/CURRENT_PROJECT_VERSION = \"${DISPLAY_VERSION}\";/" "${XCODE_PROJECT}"