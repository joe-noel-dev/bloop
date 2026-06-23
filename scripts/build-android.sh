#!/usr/bin/env bash

set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
core_dir="${project_root}/core"
output_root="${project_root}/target/android"
library_name="libbloop.so"
build_profile="${BUILD_PROFILE:-release}"
if [ $# -eq 0 ]; then
  targets=("aarch64-linux-android" "x86_64-linux-android")
else
  targets=("$@")
fi

mkdir -p "${output_root}/jniLibs"

android_abi_for_target() {
  case "$1" in
    aarch64-linux-android)
      printf '%s\n' "arm64-v8a"
      ;;
    x86_64-linux-android)
      printf '%s\n' "x86_64"
      ;;
    armv7-linux-androideabi)
      printf '%s\n' "armeabi-v7a"
      ;;
    *)
      echo "Unsupported Android target: $1" >&2
      exit 1
      ;;
  esac
}

ensure_target_installed() {
  local target="$1"
  if ! rustup target list --installed | grep -qx "${target}"; then
    rustup target add "${target}"
  fi
}

build_target() {
  local target="$1"
  local abi
  abi="$(android_abi_for_target "${target}")"
  local target_env_suffix
  local target_cc_env
  local target_ar_env
  local linker_path
  local ar_path

  ensure_target_installed "${target}"

  target_env_suffix="$(printf '%s' "${target}" | tr '[:lower:]-' '[:upper:]_')"
  target_cc_env="CC_${target//-/_}"
  target_ar_env="AR_${target//-/_}"
  linker_path="$(${project_root}/scripts/android-toolchain-path.sh "${target}" clang)"
  ar_path="$(${project_root}/scripts/android-toolchain-path.sh "${target}" ar)"

  local cargo_args=(build --target "${target}" --no-default-features --features midi)
  if [ "${build_profile}" = "release" ]; then
    cargo_args+=(--release)
  fi

  (
    cd "${core_dir}"
    env \
      "${target_cc_env}=${linker_path}" \
      "${target_ar_env}=${ar_path}" \
      "CARGO_TARGET_${target_env_suffix}_LINKER=${linker_path}" \
      "CARGO_TARGET_${target_env_suffix}_AR=${ar_path}" \
      cargo "${cargo_args[@]}"
  )

  local artifact_dir="${core_dir}/target/${target}/${build_profile}"
  local source_lib="${artifact_dir}/${library_name}"
  if [ ! -f "${source_lib}" ]; then
    echo "Expected Android library not found: ${source_lib}" >&2
    exit 1
  fi

  local abi_output_dir="${output_root}/jniLibs/${abi}"
  mkdir -p "${abi_output_dir}"
  cp "${source_lib}" "${abi_output_dir}/${library_name}"
}

for target in "${targets[@]}"; do
  build_target "${target}"
done

echo "Android libraries staged in ${output_root}/jniLibs"