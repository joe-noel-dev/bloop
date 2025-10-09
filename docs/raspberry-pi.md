# Run server on Raspberry Pi

## Set up Linux

- Use imager to enable ssh & add Wi-Fi credentials
- Copy key using `ssh-copy-id -i ~/.ssh/key.pub user@server`
- `sudo apt-get update`
- `sudo apt-get install -y jackd`
- Add user to realtime group. Much of this may have been done by jackd
  - [https://jackaudio.org/faq/linux_rt_config.html](https://jackaudio.org/faq/linux_rt_config.html)
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

Install to Raspberry Pi:

```sh
scp ./target/release/bloop-core joe@bloopi.local:/home/joe/
```

## `/home/joe/.config/openbox/autostart`

```sh
#!/bin/bash

xset -dpms
xset s off
xset s noblank


cd /home/joe/bloop
/usr/bin/bloop-core &
```

## `/etc/systemd/system/jackd.service`

```toml
[Unit]
Description=JACK Audio Server
After=sound.target local-fs.target

[Service]
Type=simple
User=joe
ExecStart=/usr/bin/jackd -R -P95 -dalsa -dhw:Pro -r44100 -p512
Restart=on-failure
LimitRTPRIO=95
LimitMEMLOCK=infinity
Environment=JACK_NO_AUDIO_RESERVATION=1

[Install]
WantedBy=default.target
```

## `/etc/lightdm/lightdm.conf`

```toml
[Seat:*]
autologin-user=joe
autologin-session=openbox
```

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
