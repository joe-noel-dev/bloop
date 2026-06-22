# Installing Bloop on Android

This guide covers sideloading the Bloop APK onto your own Android device directly from a GitHub Release.

## Prerequisites

- An Android device running Android 8.0 (API 26) or later
- USB cable (for `adb` method) or a way to transfer a file to the device

## Step 1: Download the APK

Go to the [GitHub Releases page](https://github.com/joe-noel-dev/bloop/releases) and download the latest `bloop-release.apk` (or similarly named `.apk` file) attached to the release.

## Step 2: Enable installation from unknown sources

Android blocks installation of APKs that didn't come from the Play Store by default.

1. Open **Settings → Apps** (on some devices: **Settings → Privacy → Install unknown apps** or **Settings → Security**).
2. Find the app you'll use to open the APK — typically **Files**, **My Files**, or your browser.
3. Toggle **Allow from this source** on.

The exact path varies by manufacturer and Android version. Searching "install unknown apps" in your Settings search bar will take you directly there.

## Option A: Install by opening the file on the device

1. Transfer the APK to your device (via USB, AirDrop-equivalent, cloud storage, etc.).
2. Open a file manager on the device and tap the APK.
3. Tap **Install** when prompted.

## Option B: Install via adb (USB)

This requires [Android SDK Platform Tools](https://developer.android.com/tools/releases/platform-tools) (`adb`) installed on your computer.

### Enable USB debugging

1. Go to **Settings → About phone** and tap **Build number** seven times to unlock Developer Options.
2. Go to **Settings → Developer options** and enable **USB debugging**.
3. Connect your device with a USB cable and accept the "Allow USB debugging?" prompt on the device.

### Run the install command

```sh
adb install path/to/bloop-release.apk
```

To upgrade an existing installation without uninstalling first:

```sh
adb install -r path/to/bloop-release.apk
```

Confirm success:

```
Success
```

## Setting up signing secrets (CI/maintainers)

The release APK is signed using a keystore stored as GitHub secrets. Add the following secrets to the repository under **Settings → Secrets and variables → Actions**:

| Secret name                  | Description                                          |
| ---------------------------- | ---------------------------------------------------- |
| `ANDROID_KEYSTORE_BASE64`    | Base64-encoded `.jks` or `.keystore` file            |
| `ANDROID_KEYSTORE_PASSWORD`  | Password for the keystore                            |
| `ANDROID_KEY_ALIAS`          | Alias of the signing key inside the keystore         |
| `ANDROID_KEY_PASSWORD`       | Password for the signing key                         |

### Generating a keystore (first-time setup)

```sh
keytool -genkeypair \
  -v \
  -keystore bloop-release.jks \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -alias bloop

# Encode it for the secret:
base64 -i bloop-release.jks | pbcopy   # macOS — pastes to clipboard
base64 bloop-release.jks               # Linux — print to stdout
```

Store `bloop-release.jks` somewhere safe and **do not commit it to the repository**.
