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
./scripts/install.sh [destination]
```
