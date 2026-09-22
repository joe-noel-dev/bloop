# Bluetooth LE peripheral

Bloop Core can expose its existing protobuf control API as a Bluetooth Low Energy peripheral on Linux. BLE runs alongside WebSocket networking and FFI: requests from every transport enter the same controller, and every connected client receives state responses from the same broadcast source.

BLE is disabled by default. Enable it with:

```sh
BLOOP_BLE_ENABLED=1 bloop-core --headless
```

Accepted enabled values are `1`, `true`, `yes`, and `on`, case-insensitively. A missing Bluetooth adapter, unavailable BlueZ daemon, or D-Bus error does not stop the core or its network server. Registration retries automatically.

## Linux requirements

The implementation uses BlueZ through D-Bus. Debian-based build environments need `libdbus-1-dev`; installed systems need BlueZ, D-Bus, and a BLE-capable adapter:

```sh
sudo apt-get install bluez dbus libdbus-1-3
sudo systemctl enable --now bluetooth
bluetoothctl power on
```

The core attempts to power on the default adapter when BLE is enabled. Check BlueZ failures with:

```sh
journalctl -u bluetooth
```

## GATT service

The peripheral advertises the local name `Bloop` and one custom primary service:

| Role | UUID | Operations |
| --- | --- | --- |
| Service | `80cf2c4a-803f-4db5-9464-ffd9ec0548c9` | Primary service |
| Requests | `e03062a5-a493-4712-9972-7616363b8fd3` | Write with response |
| Responses | `479f504d-373c-4469-9940-fe169b3c5c9f` | Notify |

Only one BLE central is served at a time. The central must subscribe to the response characteristic before writing requests. WebSocket and FFI clients can remain connected concurrently.

## Message framing

The characteristic values carry the same serialized `bloop.Request` and `bloop.Response` messages used by WebSocket and FFI. Messages are split to fit the negotiated GATT MTU. Every chunk starts with this 11-byte header:

| Offset | Size | Value |
| --- | ---: | --- |
| 0 | 1 | Protocol version, currently `1` |
| 1 | 2 | Message ID, unsigned little-endian |
| 3 | 4 | Zero-based chunk index, unsigned little-endian |
| 7 | 4 | Total chunk count, unsigned little-endian |
| 11 | remaining | Protobuf payload bytes |

Chunks may arrive out of order. Receivers must group them by message ID, accept byte-identical duplicates, reject conflicting duplicates, and concatenate payloads by chunk index. Bloop accepts messages up to 16 MiB, up to 262,144 chunks, and abandons an incomplete message after 15 seconds without a chunk.

Large messages work but BLE bandwidth is limited. Clients uploading audio should send multiple reasonably sized `UploadRequest` messages rather than relying on a single very large protobuf.

## Per-client progress rate

The core still calculates playback progress at up to 60 Hz, while each client session receives progress at 10 Hz by default. A standalone request can change only the requesting connection:

```proto
Request {
  configure_client {
    progress_updates_per_second: 30
  }
}
```

Valid values are 0 through 60. Zero disables periodic progress. Terminal zero progress is always delivered so a client can reset its display. The core returns a `Response.client_configuration` acknowledgement with the effective rate; invalid values return `Response.error` without changing the existing rate.

The shipped iOS and Android apps request 60 Hz for their local FFI sessions. WebSocket and BLE clients remain at 10 Hz unless they configure themselves.

## Verification client

On Linux, build and run the included client:

```sh
cd core
cargo run --example ble_client -- get-all
cargo run --example ble_client -- get-all --progress-hz 30
cargo run --example ble_client -- --request request.pb --output responses.bin
```

`--request` accepts one serialized `bloop.Request`. `--output` writes each received response as a little-endian `u32` byte length followed by the serialized `bloop.Response`.

## Troubleshooting

- If Bloop is not discoverable, confirm `BLOOP_BLE_ENABLED` is set on the core process, `bluetoothctl show` reports `Powered: yes`, and the adapter supports LE advertising.
- If registration fails with a D-Bus authorization error, run Bloop as a user permitted to manage the system BlueZ service or install an appropriate system policy. Do not grant broader D-Bus access than the deployment needs.
- A request rejected with `NotPermitted` usually means the central wrote before subscribing to response notifications, or another central already owns the BLE session.
- After a response-stream lag error, send `Get ALL` to resynchronise state.
- For framing errors, use the negotiated characteristic value limit rather than the nominal ATT MTU, include the 11-byte header in that limit, and do not interleave chunks from different messages.

## Security

The first version is unpaired and does not add application-layer authentication or encryption. Enabling it allows anyone in Bluetooth radio range to issue every Bloop command, including project and upload operations. Only enable BLE in environments where that exposure is acceptable.
