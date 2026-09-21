# Install Bloop Core on Linux

This guide installs Bloop Core from source on a dedicated Ubuntu Server machine
with:

- direct ALSA audio;
- optional USB or Bluetooth MIDI;
- a single-screen Weston kiosk; and
- remote administration over SSH.

It was tested on Ubuntu Server 26.04 LTS. It does not require a desktop
environment, display manager, JACK server, fixed hostname, or particular audio
interface.

## 1. Install system packages

```sh
sudo apt update
sudo apt install -y \
  alsa-utils \
  bluez \
  build-essential \
  curl \
  git \
  libasound2-dev \
  libjack-jackd2-dev \
  openssh-server \
  pkg-config \
  protobuf-compiler \
  seatd \
  weston
```

`protobuf-compiler` is required by the local Rust build. JACK itself is not used
by this setup, but the JACK development library is needed because the Linux
binary is compiled with Bloop's optional JACK backend.

Enable SSH and the seat-management daemon:

```sh
sudo systemctl enable --now ssh seatd
```

## 2. Give the service access to audio and video devices

Run the following as the non-root account that will run Bloop:

```sh
sudo usermod -aG audio,video "$(id -un)"
```

Group changes apply to new login sessions. Log out and back in before using the
manual diagnostic commands later in this guide. The kiosk service also declares
these groups explicitly.

Confirm that ALSA can see the connected audio devices:

```sh
aplay -l
```

If this works only with `sudo`, the current login session has not picked up the
`audio` group yet.

## 3. Install Rust and build Bloop Core

Install the current stable Rust toolchain with rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
```

Clone and build Bloop Core:

```sh
mkdir -p "$HOME/src"
git clone https://github.com/joe-noel-dev/bloop.git "$HOME/src/bloop"
cd "$HOME/src/bloop/core"
cargo build --release
```

Install the binary:

```sh
sudo install -m 0755 target/release/bloop-core /usr/local/bin/bloop-core
```

## 4. Create the Bloop data directory

```sh
mkdir -p "$HOME/bloop/logs"
```

Bloop stores its preferences, projects, samples, and logs under this directory.
See [PREFERENCES.md](PREFERENCES.md) for the preference-file format. Audio and
MIDI devices can also be selected from a connected Bloop client after the core
is running.

## 5. Configure the kiosk display

Find the first connected DRM output and its graphics device. Keep this shell
open until the service file has been created in the next step:

```sh
DRM_CONNECTOR_PATH="$({
  for STATUS_PATH in /sys/class/drm/card*-*/status; do
    if [ "$(cat "$STATUS_PATH")" = connected ]; then
      dirname "$STATUS_PATH"
      break
    fi
  done
})"

if [ -z "$DRM_CONNECTOR_PATH" ]; then
  echo 'No connected display output found'
else
  BLOOP_DRM_CONNECTOR="$(basename "$DRM_CONNECTOR_PATH")"
  BLOOP_DRM_CARD="${BLOOP_DRM_CONNECTOR%%-*}"
  BLOOP_OUTPUT="${BLOOP_DRM_CONNECTOR#*-}"
  printf 'Using graphics device %s and output %s\n' \
    "$BLOOP_DRM_CARD" "$BLOOP_OUTPUT"
fi
```

Create a Weston configuration. A fixed 1080p mode avoids first-frame failures
seen with some older integrated graphics and 4K displays:

```sh
tee "$HOME/bloop/weston.ini" >/dev/null <<EOF
[output]
name=$BLOOP_OUTPUT
mode=1920x1080@60
EOF
```

If no output was found, check that the screen is connected and powered on, then
run the detection command again.

## 6. Create the kiosk service

Set variables for the account running this installation:

```sh
BLOOP_USER="$(id -un)"
BLOOP_USER_HOME="$(getent passwd "$BLOOP_USER" | cut -d: -f6)"
```

Create the service:

```sh
sudo tee /etc/systemd/system/bloop-kiosk.service >/dev/null <<EOF
[Unit]
Description=Bloop Core kiosk
Requires=seatd.service
After=seatd.service network-online.target sound.target
Wants=network-online.target
Conflicts=getty@tty1.service

[Service]
Type=simple
User=$BLOOP_USER
SupplementaryGroups=audio video
RuntimeDirectory=bloop-runtime
RuntimeDirectoryMode=0700
Environment=XDG_RUNTIME_DIR=/run/bloop-runtime
Environment=LIBSEAT_BACKEND=seatd
Environment=BLOOP_HOME=$BLOOP_USER_HOME/bloop
Environment=ICED_BACKEND=tiny-skia
ExecStart=/usr/bin/weston --backend=drm --drm-device=$BLOOP_DRM_CARD --renderer=pixman --shell=kiosk --idle-time=0 --config=$BLOOP_USER_HOME/bloop/weston.ini --continue-without-input --log=$BLOOP_USER_HOME/bloop/logs/weston.log -- /usr/local/bin/bloop-core
Restart=always
RestartSec=2
KillMode=control-group

[Install]
WantedBy=multi-user.target
EOF
```

The service uses seatd rather than a display manager. If Weston exits, systemd
stops the whole process group, including Bloop Core, before restarting the
kiosk. This prevents a headless Bloop process from retaining the audio device or
network port after a display failure.

Disable the login prompt on the kiosk virtual terminal, then enable the service:

```sh
sudo systemctl daemon-reload
sudo systemctl disable --now getty@tty1.service
sudo systemctl enable --now bloop-kiosk.service
```

The UI should now appear without a reboot. Check the service and follow its log
over SSH with:

```sh
systemctl status bloop-kiosk.service --no-pager
journalctl -u bloop-kiosk.service -f
```

Weston's separate display log is stored at:

```text
~/bloop/logs/weston.log
```

Reboot once to verify automatic startup:

```sh
sudo reboot
```

## 7. Configure Bluetooth MIDI

USB MIDI devices need no Bluetooth setup. To pair a Bluetooth MIDI device, run:

```sh
bluetoothctl
```

At the `bluetoothctl` prompt:

```text
power on
agent on
default-agent
scan on
```

Wait for the controller to appear, then use its reported address:

```text
pair DEVICE_ADDRESS
trust DEVICE_ADDRESS
connect DEVICE_ADDRESS
scan off
quit
```

Confirm that ALSA has created a MIDI port:

```sh
aconnect -l
```

Enable the reported port from Bloop's MIDI settings. Bloop matches enabled MIDI
devices against their full ALSA port names, so do not assume the name will be
the same as the product label.

## Updating Bloop Core

```sh
cd "$HOME/src/bloop"
git pull --ff-only
cd core
cargo build --release
sudo install -m 0755 target/release/bloop-core /usr/local/bin/bloop-core
sudo systemctl restart bloop-kiosk.service
```

## Manual display test over SSH

Stop the kiosk service before running Weston manually:

```sh
sudo systemctl stop bloop-kiosk.service
```

Then run the same known-good display stack in the foreground:

```sh
XDG_RUNTIME_DIR=/run/user/"$(id -u)" \
LIBSEAT_BACKEND=seatd \
BLOOP_HOME="$HOME/bloop" \
ICED_BACKEND=tiny-skia \
/usr/bin/weston \
  --backend=drm \
  --renderer=pixman \
  --shell=kiosk \
  --idle-time=0 \
  --config="$HOME/bloop/weston.ini" \
  --continue-without-input \
  --log="$HOME/bloop/logs/weston-manual.log" \
  -- /usr/local/bin/bloop-core
```

Press `Ctrl+C` to stop it, then restore the service:

```sh
pkill -x bloop-core 2>/dev/null || true
sudo systemctl start bloop-kiosk.service
```

## Renderer choices

The service deliberately uses two software-rendering settings:

- `--renderer=pixman` selects Weston's software compositor.
- `ICED_BACKEND=tiny-skia` selects Iced's software UI renderer.

They require no extra packages beyond the dependencies already included in the
Bloop binary and Weston. This combination is the conservative default for a
dedicated kiosk, especially on older integrated graphics. GPU rendering can be
tested later by removing one setting at a time, but keep the software defaults
until the replacement has survived both a service restart and a reboot.
