# Core

The core audio server built in Rust. Handles audio processing, looping, and MIDI control.

## Build

```sh
cargo build
```

## Build for Android

Use no default features for Android so desktop UI and MIDI remain disabled:

```sh
cargo build --target aarch64-linux-android --no-default-features
```

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
