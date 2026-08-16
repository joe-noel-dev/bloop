# Run Bloop Core on Raspberry Pi

## Current `bloopi` kiosk configuration

`bloopi` runs Raspberry Pi OS with the Labwc Wayland desktop. Bloop Core is
started after the desktop session is available, rather than by a system-wide
systemd unit.

The installed binary is `/usr/bin/bloop-core` (there is no
`/usr/local/bloop-core` on the device). Labwc starts it from
`/home/joe/.config/labwc/autostart`:

```sh
wlr-randr --output HDMI-A-1 --scale 1.8
sleep 2
export WGPU_BACKEND=vulcan
bloop-core
```

The process uses ALSA directly: it has `/dev/snd/pcmC3D0p` (the Scarlett 4i4)
open. `jackd` is not installed or running, so it is not required for this
configuration. PipeWire and WirePlumber may run as part of the desktop session,
but Bloop Core is not routing audio through them.

## Set up Linux

- Use imager to enable ssh & add Wi-Fi credentials
- Copy key using `ssh-copy-id -i ~/.ssh/key.pub user@server`
- `sudo apt-get update`
- Create the configuration directory (`~/bloop`)
- Copy settings
  - `rsync -r ~/bloop [user]@[remote]:[remote/path/to/home]`
- Create a static /dev/ port for serial
  - [https://msadowski.github.io/linux-static-port/](https://msadowski.github.io/linux-static-port/)

## Modify Wi-Fi credentials

Modify the `wpa_supplicant` configuration file:

```sh
sudo nano /etc/wpa_supplicant/wpa_supplicant.conf
```

Add credentials:

```text
network={
    ssid="Your_SSID"
    psk="Your_WiFi_Password"
    key_mgmt=WPA-PSK
}
```

Restart service:

```sh
sudo systemctl restart dhcpcd
```

## Cross-compile

Cross compile from Mac:

```sh
./scripts/cross-compile.sh
```

Install the binary on the current kiosk:

```sh
scp ./target/release/bloop-core joe@bloopi.local:/home/joe/bloop-core
ssh joe@bloopi.local 'sudo install -m 755 /home/joe/bloop-core /usr/bin/bloop-core'
```

Restart the desktop session or reboot to pick up the new binary.

## `/home/joe/.config/labwc/autostart`

```sh
wlr-randr --output HDMI-A-1 --scale 1.8
sleep 2
export WGPU_BACKEND=vulcan
bloop-core
```

Do not background the last command: Labwc keeps it as part of its session and
will terminate it when the desktop session ends.

## Optional JACK configuration

JACK remains an optional Linux backend. Install and configure `jackd` only when
the Bloop audio preference `useJack` is enabled. With its default value of
`false`, Bloop Core selects CPAL's default host, which is direct ALSA on the
current kiosk.

## Preferences `~/bloop/preferences.json`

```json
{
  "audio": {
    "outputChannelCount": 4,
    "sampleRate": 48000
  },
  "pedal": {
    "serialPath": "/dev/bloop_pedal"
  },
  "switch": {
    "mappings": [
      {
        "pin": 4,
        "gesture": "press",
        "action": "toggleLoop"
      },
      {
        "pin": 17,
        "gesture": "release",
        "action": "nextSong"
      },
      {
        "pin": 17,
        "gesture": "hold",
        "action": "previousSong"
      },
      {
        "pin": 22,
        "gesture": "press",
        "action": "togglePlay"
      }
    ]
  }
}
```
