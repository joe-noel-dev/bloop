# Audio Device Selection Work Breakdown

This document breaks the richer audio-device experience into a sequence of small,
reviewable PRs. The goals are:

- Browse available output devices in the app preferences UI
- Browse available sample rates for the selected device
- Use the device's native output channel count rather than a hardcoded value
- Switch device, sample rate, and channel layout at runtime without restarting the
  core or the app
- Surface in the UI when the audio callback is not running so the user notices

Audio continues to live entirely in the Rust core. The proto API
(`api/bloop.proto`) is the only contract: it exposes the list of devices, the
current device status, and the commands needed to switch device. Audio is allowed
to stop while a device switch is in progress, as long as the playback state is
broadcast so the UI reflects that audio has stopped.

## Current state (for context)

- `core/src/audio/controller.rs` owns the `AudioController`, which is constructed
  once at startup in `core/src/control/main.rs` and lives for the lifetime of the
  core. It owns the `rawdio` `Context`, mixer, metronome, samplers, sequencer,
  and an `AudioProcessRunner` that holds the `cpal` stream.
- `core/src/audio/process.rs` builds the `cpal::Stream` from `AudioPreferences`
  and falls back to a `NoopProcess` if hardware init fails. Once built, the
  stream cannot be reconfigured.
- `AudioPreferences` (in `api/bloop.proto`) exposes `output_device` (string),
  `sample_rate`, `buffer_size`, `output_channel_count`, `use_jack`,
  `main_channel_offset`, `click_channel_offset`. `output_channel_count` is
  hardcoded by the user in preferences.
- iOS (`ios/source/views/PreferencesView.swift`) and Android
  (`android/app/src/main/java/com/joenoel/bloop/ui/PreferencesScreen.kt`)
  currently render the output device as a plain text field and the sample rate
  as a numeric field. There is no concept of "available devices" or "callback
  running" on the wire.

## Phase 1: Restartable audio in the core

### PR 1: Make `AudioController` stoppable and recreatable

- Refactor `AudioController` so that the realtime audio backend (the
  `AudioProcessRunner` plus the `rawdio` `Context`) can be torn down and rebuilt
  without losing project state, samples cache, or the response broadcaster
- Split the controller into a long-lived part (project, samples, response
  channel, conversion pipeline, preferences) and a short-lived "audio engine"
  part (context, mixer, metronome, samplers, sequencer, realtime process)
- Add `start_audio` / `stop_audio` methods on `AudioController` that build or
  drop the audio engine; samples are re-added to the new engine on start
- Ensure `stop_audio` cleanly drops the `cpal::Stream` and the rawdio context so
  the OS releases the device
- Keep `AudioController::run` working when the engine is stopped (no panics,
  no busy loop)

### PR 2: Audio engine status and playback reset on stop

- Add an internal `AudioEngineState { Stopped, Running, Failed { reason } }` in
  the core, owned by `AudioController`
- When transitioning to `Stopped` or `Failed`, reset playback and broadcast a
  `PlaybackState` with `PLAYING = STOPPED` and cleared progress so the UI
  reflects that audio is no longer playing
- Add a unit test that drives `start_audio` -> `stop_audio` -> `start_audio`
  with the dummy audio backend and asserts the broadcast playback state

### PR 3: Detect a stalled audio callback

- Track callback liveness in `audio/process.rs` (for example: increment an
  atomic counter from inside the cpal callback)
- From `AudioController::run`, check that the counter advances within a
  reasonable window while the engine is supposed to be running; if it does not,
  mark the engine as `Failed { reason: "callback not running" }`
- Treat the dummy and noop processes appropriately (dummy reports live, noop
  reports stalled)

## Phase 2: Proto API for devices and status

### PR 4: Add audio device discovery types to `bloop.proto`

- Add a new `AudioDevice` message describing one output device: id/name,
  whether it is the system default, supported sample rates, supported channel
  counts, supported buffer-size range
- Add a new `AudioDevices` message (a list of `AudioDevice` plus the current
  host name) and wire it into `Response`
- Add `AUDIO_DEVICES` to the `Entity` enum so it participates in the existing
  Get-by-entity pattern
- Regenerate the proto bindings on the Rust, Swift, and Kotlin sides

### PR 5: Implement device discovery in the core

- Add `core/src/audio/devices.rs` that enumerates `cpal` output devices and
  their supported configs and produces an `AudioDevices` proto message
- Route a `Get { entity: AUDIO_DEVICES }` request through `control/main.rs` so
  apps can fetch the list on demand
- Broadcast the device list once on startup and whenever the core notices a
  device topology change (deferred to PR 11 if needed; initial implementation
  may rely on apps polling on focus)

### PR 6: Add audio status to the proto and broadcast it

- Add an `AudioStatus` message: current device id/name, current sample rate,
  current channel count, current buffer size, engine state
  (`STOPPED | RUNNING | FAILED`), and an optional error string
- Wire `AudioStatus` into `Response`
- Add core support for broadcasting `AudioStatus` whenever the engine state
  transitions (start, stop, callback stall, init failure) and on initial
  connect
- Add `AUDIO_STATUS` to the `Entity` enum for explicit fetch

### PR 7: Add audio control commands to the proto

- Add an `AudioControlRequest` with methods such as `START`, `STOP`, `RESTART`
  - `START` and `RESTART` apply the current `AudioPreferences` before bringing
    the engine up
- Wire `AudioControlRequest` into the top-level `Request` message
- Hook it up in `control/main.rs` to call the new `start_audio` / `stop_audio`
  on `AudioController`

## Phase 3: Preferences cleanup

### PR 8: Honour preferences updates as a device switch

- When a `UpdateRequest { preferences }` arrives and the audio section has
  changed (device, sample rate, channel offsets, buffer size, jack toggle),
  trigger a `stop_audio` -> `start_audio` cycle inside `AudioController`
- Persist preferences with the existing `write_preferences` flow only after the
  new engine has come up (or has failed and reported `Failed`)
- Broadcast `AudioStatus` and the reset `PlaybackState` around the switch

### PR 9: Use the device's native output channel count

- Stop treating `output_channel_count` as user input. Instead, query the
  selected device's default/native channel count when building the engine and
  report it back via `AudioStatus`
- Update `select_stream_config` in `audio/process.rs` accordingly on each
  platform
- Validate `main_channel_offset` and `click_channel_offset` against the native
  channel count, clamping with a log warning if needed
- Remove `output_channel_count` from `AudioPreferences` in `bloop.proto`,
  `default_audio_preferences`, `validate_preferences`, and the iOS/Android
  preference UIs and persisted preferences files

## Phase 4: App UI for device selection

### PR 10: iOS device selection UI

- Replace the free-text "Output Device" field in `PreferencesView.swift` with a
  picker populated from the `AudioDevices` response
- Replace the free-text sample-rate field with a picker whose options come from
  the selected device's supported sample rates
- Fetch `AUDIO_DEVICES` and subscribe to `AudioStatus` on view appear; refresh
  on pull-to-refresh
- Show the current device, sample rate, and channel count read from
  `AudioStatus`
- Show a clear inline warning when `AudioStatus.engine_state` is `FAILED` or
  `STOPPED`, including any error text, surfaced both in the preferences screen
  and on the main transport area so it draws attention
- Add an explicit "Restart audio" button that sends `AudioControlRequest`

### PR 11: Android device selection UI

- Mirror the iOS work in `android/app/src/main/java/com/joenoel/bloop/ui/PreferencesScreen.kt`
  using Jetpack Compose dropdowns (`ExposedDropdownMenuBox`) for device and
  sample rate
- Subscribe to `AudioStatus` via the existing state store and middleware
- Surface the same `FAILED` / `STOPPED` callback warning on the preferences
  screen and on the main transport area
- Add a "Restart audio" action

### PR 12: Editor device selection UI (optional, follow-up)

- Bring the same picker UX to the desktop editor (`editor/`), reusing the
  shared proto schema and the existing theme tokens
- Show the audio status banner in the editor when the callback is not running

## Phase 5: Polish and resilience

### PR 13: Hot-plug device topology updates

- Detect device add/remove events from `cpal` where the platform supports it
  (or poll on a slow interval as a fallback) and rebroadcast `AudioDevices`
- If the currently selected device disappears, transition the engine to
  `Stopped` and broadcast `AudioStatus` so the apps can prompt the user

### PR 14: Tests and CI coverage

- Add core unit tests for: start/stop cycles, preference-driven restart,
  device-disappears-while-running, and callback-stall detection (using the
  dummy backend)
- Add iOS and Android UI tests that assert the warning banner appears when
  `AudioStatus.engine_state` is `FAILED`

## Suggested order

1. PRs 1-3: Restartable audio inside the core
2. PRs 4-7: Proto API for devices, status, and control commands
3. PRs 8-9: Preferences cleanup and native channel count
4. PRs 10-11: iOS and Android device-selection UI
5. PRs 12-14: Editor support, hot-plug, and tests

## Parallel work opportunities

- PRs 4 and 5 can be authored together once the proto shape is agreed
- PR 10 (iOS) and PR 11 (Android) can be done in parallel once PRs 4-7 land
- PR 13 (hot-plug) can proceed independently of the UI PRs
