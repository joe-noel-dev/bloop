#!/usr/bin/env bash

set -euo pipefail

target_triple="${1:?target triple is required}"
tool_name="${2:?tool name is required}"

ndk_root="${ANDROID_NDK_HOME:-${ANDROID_NDK_ROOT:-}}"

if [ -z "${ndk_root}" ]; then
  sdk_root="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
  ndk_dir="${sdk_root}/ndk"

  if [ -d "${ndk_dir}" ]; then
<<<<<<< HEAD
    latest_ndk="$(find "${ndk_dir}" -mindepth 1 -maxdepth 1 -type d ! -name 'ndk-bundle' | sort -V | tail -n 1)"
=======
    latest_ndk="$(find "${ndk_dir}" -mindepth 1 -maxdepth 1 -type d | sort | tail -n 1)"
>>>>>>> f49c8f6 (Build Rust core for Android)
    if [ -n "${latest_ndk}" ]; then
      ndk_root="${latest_ndk}"
    fi
  fi
fi

if [ -z "${ndk_root}" ] || [ ! -d "${ndk_root}" ]; then
  echo "Unable to locate Android NDK. Set ANDROID_NDK_HOME, ANDROID_NDK_ROOT, ANDROID_SDK_ROOT, or ANDROID_HOME." >&2
  exit 1
fi

host_tag=""
for candidate in darwin-x86_64 darwin-arm64 linux-x86_64 windows-x86_64; do
  if [ -d "${ndk_root}/toolchains/llvm/prebuilt/${candidate}" ]; then
    host_tag="${candidate}"
    break
  fi
done

if [ -z "${host_tag}" ]; then
  echo "Unable to locate Android NDK prebuilt toolchain under ${ndk_root}" >&2
  exit 1
fi

bin_dir="${ndk_root}/toolchains/llvm/prebuilt/${host_tag}/bin"

<<<<<<< HEAD
clang_triple="${target_triple}"
if [ "${target_triple}" = "armv7-linux-androideabi" ]; then
  clang_triple="armv7a-linux-androideabi"
fi

case "${tool_name}" in
  clang)
    api_level="${ANDROID_MIN_SDK_VERSION:-26}"
    tool_path="${bin_dir}/${clang_triple}${api_level}-clang"
    ;;
  clang++)
    api_level="${ANDROID_MIN_SDK_VERSION:-26}"
    tool_path="${bin_dir}/${clang_triple}${api_level}-clang++"
=======
case "${tool_name}" in
  clang)
    api_level="${ANDROID_MIN_SDK_VERSION:-26}"
    tool_path="${bin_dir}/${target_triple}${api_level}-clang"
    ;;
  clang++)
    api_level="${ANDROID_MIN_SDK_VERSION:-26}"
    tool_path="${bin_dir}/${target_triple}${api_level}-clang++"
>>>>>>> f49c8f6 (Build Rust core for Android)
    ;;
  ar)
    tool_path="${bin_dir}/llvm-ar"
    ;;
  ranlib)
    tool_path="${bin_dir}/llvm-ranlib"
    ;;
  strip)
    tool_path="${bin_dir}/llvm-strip"
    ;;
  *)
    echo "Unsupported tool name: ${tool_name}" >&2
    exit 1
    ;;
esac

if [ ! -x "${tool_path}" ]; then
  echo "Expected Android tool not found: ${tool_path}" >&2
  exit 1
fi

printf '%s\n' "${tool_path}"