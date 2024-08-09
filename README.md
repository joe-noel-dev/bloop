# bloop

Backing looper

Consists of:

- Audio server, built in Rust: [/src](src)
- iOS application: [/ios](ios)
- Firmware for a control pedal, running on an Arduino: [/pedal](pedal)
- A React-based editor: [/editor](editor)

## Server

Build:

```sh
cargo build
```

Test:

```sh
cargo test
```

Run:

```sh
cargo run
```
