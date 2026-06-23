# Core

The core audio server built in Rust. Handles audio processing, looping, and MIDI control.

## Build

```sh
cargo build
```

## Build for Android

Use no default features for Android so the desktop UI remains disabled. The `midi` feature is enabled for Android builds.

Run the following from the **repo root**:

```sh
./scripts/build-android.sh
```

The script builds `aarch64-linux-android` by default and stages `libbloop.so` under `target/android/jniLibs/arm64-v8a/`.

Optional targets can be passed explicitly, for example (run from repo root):

```sh
./scripts/build-android.sh aarch64-linux-android x86_64-linux-android armv7-linux-androideabi
```

Android builds look for the NDK in this order: `ANDROID_NDK_HOME`, `ANDROID_NDK_ROOT`, then the latest version under `ANDROID_SDK_ROOT`, `ANDROID_HOME`, or `~/Library/Android/sdk/ndk`.

The default Android API level is `31`. Override it with `ANDROID_MIN_SDK_VERSION` if needed.

Android runtime expectation: the app layer must set `BLOOP_HOME` before core startup. On Android, core now treats a missing `BLOOP_HOME` as a startup error.

## Test

```sh
cargo test
```

## Run

```sh
cargo run
```

## Run on Raspberry Pi

See instructions in [raspberry-pi](../docs/raspberry-pi.md)
