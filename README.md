# bloop

Backing looper

Consists of:

- Audio server, built in Rust: [/core](core)
- iOS application: [/ios](ios)
- Firmware for a control pedal, running on an Arduino: [/pedal](pedal)
- A React-based editor: [/editor](editor)

## Server

Build:

```sh
cd core
cargo build
```

Test:

```sh
cd core
cargo test
```

Run:

```sh
cd core
cargo run
```

### Run on Raspberry Pi

See instructions in [raspberry-pi](./docs/raspberry-pi.md)
