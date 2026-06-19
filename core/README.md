# Core

The core audio server built in Rust. Handles audio processing, looping, and MIDI control.

## Build

```sh
cargo build
```

## Build for Android

Use no default features for Android so desktop UI and MIDI remain disabled:

```sh
../scripts/build-android.sh
```

The script builds `aarch64-linux-android` by default and stages `libbloop.so` under `target/android/jniLibs/arm64-v8a/`.

Optional targets can be passed explicitly, for example:

```sh
../scripts/build-android.sh aarch64-linux-android x86_64-linux-android armv7-linux-androideabi
```

Android builds look for the NDK in this order: `ANDROID_NDK_HOME`, `ANDROID_NDK_ROOT`, then the latest version under `ANDROID_SDK_ROOT`, `ANDROID_HOME`, or `~/Library/Android/sdk/ndk`.

The default Android API level is `26` because the current audio stack links against `aaudio`. Override it with `ANDROID_MIN_SDK_VERSION` if the Android app build needs a newer API level.

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
