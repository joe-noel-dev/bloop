#!/usr/bin/env bash

set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tool_path="$(${project_root}/scripts/android-toolchain-path.sh aarch64-linux-android clang)"

exec "${tool_path}" "$@"