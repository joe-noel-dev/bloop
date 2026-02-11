# Preferences Configuration

This document describes the available settings in `preferences.json` for the Bloop core.

## Overview

Preferences are stored in `~/bloop/preferences.json` (or `$BLOOP_HOME/preferences.json` if the environment variable is set). The file uses JSON format and is read on startup.

## File Location

| Platform | Default Path |
|----------|--------------|
| macOS/Linux | `~/bloop/preferences.json` |
| iOS | `~/Documents/bloop/preferences.json` |
| Custom | `$BLOOP_HOME/preferences.json` |

## Structure

The preferences file has three main sections:

```json
{
  "audio": { ... },
  "midi": { ... },
  "switch": { ... }
}
```

## Audio Preferences

Configure audio output settings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `outputDevice` | string | `""` | Name of the audio output device |
| `sampleRate` | number | `48000` | Sample rate in Hz (valid: 1–192000) |
| `bufferSize` | number | `512` | Audio buffer size in samples (valid: 1–8192) |
| `outputChannelCount` | number | `2` | Number of output channels (valid: 1–64) |
| `useJack` | boolean | `false` | Use JACK audio server (Linux only) |
| `mainChannelOffset` | number | `0` | Output channel offset for main audio |
| `clickChannelOffset` | number | `2` | Output channel offset for click/metronome |

### Example

```json
{
  "audio": {
    "outputDevice": "Built-in Output",
    "sampleRate": 48000,
    "bufferSize": 512,
    "outputChannelCount": 4,
    "useJack": false,
    "mainChannelOffset": 0,
    "clickChannelOffset": 2
  }
}
```

### Validation

Invalid values are automatically reset to defaults:

- `outputChannelCount`: Reset to 2 if 0 or > 64
- `bufferSize`: Reset to 512 if 0 or > 8192
- `sampleRate`: Reset to 48000 if 0 or > 192000

## MIDI Preferences

Configure MIDI input settings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `inputDevice` | string | `""` | Name of the MIDI input device |

### Example

```json
{
  "midi": {
    "inputDevice": "USB MIDI Controller"
  }
}
```

## Switch Preferences

Configure GPIO switch/pedal mappings for hardware control (e.g., Raspberry Pi).

| Field | Type | Description |
|-------|------|-------------|
| `mappings` | array | List of switch mapping configurations |

### Switch Mapping

Each mapping in the `mappings` array has:

| Field | Type | Description |
|-------|------|-------------|
| `pin` | number | GPIO pin number |
| `gesture` | string | Trigger gesture type |
| `action` | string | Action to perform |

### Gestures

| Value | Description |
|-------|-------------|
| `press` | Triggered when switch is pressed down |
| `release` | Triggered when switch is released |
| `hold` | Triggered when switch is held down |

### Actions

| Value | Description |
|-------|-------------|
| `previousSong` | Go to the previous song |
| `nextSong` | Go to the next song |
| `previousSection` | Go to the previous section |
| `nextSection` | Go to the next section |
| `queueSelected` | Queue the selected song/section |
| `toggleLoop` | Toggle loop mode |
| `togglePlay` | Toggle playback (play/stop) |

### Example

```json
{
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

## Complete Example

```json
{
  "audio": {
    "outputDevice": "USB Audio Device",
    "sampleRate": 48000,
    "bufferSize": 256,
    "outputChannelCount": 4,
    "useJack": false,
    "mainChannelOffset": 0,
    "clickChannelOffset": 2
  },
  "midi": {
    "inputDevice": "MIDI Keyboard"
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
        "pin": 22,
        "gesture": "press",
        "action": "togglePlay"
      }
    ]
  }
}
```

## Notes

- All fields are optional; missing fields use default values
- Unknown fields are ignored (forward compatibility)
- Changes require restarting the core to take effect
- The iOS app provides a UI for editing preferences without manual JSON editing
