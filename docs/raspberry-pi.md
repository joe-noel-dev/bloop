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

Restart Bloop Core to pick up the new binary; a full Raspberry Pi reboot is not
needed. In VS Code, run `Core | Restart on bloopi` (or `Core | Build and install
on bloopi`, which includes the restart).

## Restart Bloop Core without rebooting the Raspberry Pi

Bloop Core is launched by Labwc rather than a systemd service. Restarting it
requires retaining the running process's Wayland environment, then starting the
replacement in that same desktop session:

```sh
ssh joe@bloopi.local 'pid=$(pgrep -xo bloop-core) || { echo "bloop-core is not running"; exit 1; }; wayland_display=$(strings /proc/$pid/environ | sed -n "s/^WAYLAND_DISPLAY=//p"); xdg_runtime_dir=$(strings /proc/$pid/environ | sed -n "s/^XDG_RUNTIME_DIR=//p"); kill $pid; while kill -0 $pid 2>/dev/null; do sleep 0.1; done; XDG_RUNTIME_DIR=$xdg_runtime_dir WAYLAND_DISPLAY=$wayland_display nohup /usr/bin/bloop-core >> /home/joe/.xsession-errors 2>&1 < /dev/null &'
```

This terminates and relaunches only `bloop-core`; it leaves Labwc, the kiosk
session, and the Raspberry Pi itself running. The command exits with an error
if Bloop Core is not already running, because its Wayland session details would
not be available to reuse.

## `/home/joe/.config/labwc/autostart`

```sh
wlr-randr --output HDMI-A-1 --scale 1.8
sleep 2
export WGPU_BACKEND=vulcan
bloop-core
```

Do not background the last command: Labwc keeps it as part of its session and
will terminate it when the desktop session ends.

## View kiosk logs over SSH

Labwc launches Bloop Core directly, so its standard output and error are written
to `/home/joe/.xsession-errors`, not the systemd journal. Follow the log from
another machine with:

```sh
ssh -t joe@bloopi.local 'tail -F ~/.xsession-errors'
```

To show only Bloop-related output:

```sh
ssh -t joe@bloopi.local 'tail -F ~/.xsession-errors | grep --line-buffered -i bloop'
```

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
