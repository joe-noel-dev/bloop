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

**Verification:**

- Cargo unit tests against the dummy audio backend that construct an
  `AudioController`, call `stop_audio`, then `start_audio`, and assert that
  `run` makes progress in both states without panicking
- Add a temporary debug request (removed in PR 7 once `AudioControlRequest`
  lands) so manual testing can drive start/stop on macOS and Linux; confirm
  via logs that the rawdio context and `cpal::Stream` are dropped and rebuilt
  and that audio resumes

### PR 2: Audio engine status and playback reset on stop

- Add an internal `AudioEngineState { Stopped, Running, Failed { reason } }` in
  the core, owned by `AudioController`
- When transitioning to `Stopped` or `Failed`, reset playback and broadcast a
  `PlaybackState` with `PLAYING = STOPPED` and cleared progress so the UI
  reflects that audio is no longer playing

**Verification:**

- Cargo unit test that starts playback in the dummy backend, calls
  `stop_audio`, and asserts the next broadcast `PlaybackState` has
  `PLAYING = STOPPED` and zeroed progress
- Cargo unit test that drives `start_audio` -> `stop_audio` -> `start_audio`
  and asserts the resulting sequence of broadcast `PlaybackState` values
- Manual: trigger an init failure (e.g. unplug the device on macOS during
  start) and confirm the iOS/Android transport UI reverts to stopped

### PR 3: Detect a stalled audio callback

- Track callback liveness in `audio/process.rs` (for example: increment an
  atomic counter from inside the cpal callback)
- From `AudioController::run`, check that the counter advances within a
  reasonable window while the engine is supposed to be running; if it does not,
  mark the engine as `Failed { reason: "callback not running" }`
- Treat the dummy and noop processes appropriately (dummy reports live, noop
  reports stalled)

**Verification:**

- Cargo unit test using the dummy backend that advances the counter and
  asserts the engine stays `Running`
- Cargo unit test using `NoopProcess` that asserts the engine transitions to
  `Failed { reason: "callback not running" }` within the stall window
- Manual: on macOS, suspend the audio process briefly (e.g. swap to a sleep
  output) and confirm the state transitions to `Failed`

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

**Verification:**

- `cargo build` for the core, Xcode build for iOS, and Gradle build for
  Android all succeed against the regenerated bindings
- Round-trip serialization unit test in Rust that encodes a sample
  `AudioDevices` message and decodes it back unchanged

### PR 5: Implement device discovery in the core

- Add `core/src/audio/devices.rs` that enumerates `cpal` output devices and
  their supported configs and produces an `AudioDevices` proto message
- Route a `Get { entity: AUDIO_DEVICES }` request through `control/main.rs` so
  apps can fetch the list on demand
- Broadcast the device list once on startup and whenever the core notices a
  device topology change (deferred to PR 13 if needed; initial implementation
  may rely on apps polling on focus)

**Verification:**

- Cargo unit test that calls the enumeration function and asserts at least
  one device is reported on the CI host (gated behind a feature flag if CI
  has no audio devices)
- Integration test against the request handler that sends
  `Get { AUDIO_DEVICES }` and asserts an `AudioDevices` response
- Manual: log the enumerated list on macOS, Linux, and Raspberry Pi and
  confirm it matches `system_profiler` / `aplay -L`

### PR 6: Add audio status to the proto and broadcast it

- Add an `AudioStatus` message: current device id/name, current sample rate,
  current channel count, current buffer size, engine state
  (`STOPPED | RUNNING | FAILED`), and an optional error string
- Wire `AudioStatus` into `Response`
- Add core support for broadcasting `AudioStatus` whenever the engine state
  transitions (start, stop, callback stall, init failure) and on initial
  connect
- Add `AUDIO_STATUS` to the `Entity` enum for explicit fetch

**Verification:**

- Cargo unit test that subscribes to the response broadcaster and asserts
  exactly one `AudioStatus` is sent on each engine transition (start ->
  stop -> start) with the expected fields
- Cargo unit test for `Get { AUDIO_STATUS }` returning the current status
- Manual: tail the response stream on iOS/Android during start/stop and
  confirm `AudioStatus` arrives with sensible values

### PR 7: Add audio control commands to the proto

- Add an `AudioControlRequest` with methods such as `START`, `STOP`, `RESTART`
  - `START` and `RESTART` apply the current `AudioPreferences` before bringing
    the engine up
- Wire `AudioControlRequest` into the top-level `Request` message
- Hook it up in `control/main.rs` to call the new `start_audio` / `stop_audio`
  on `AudioController`

**Verification:**

- Cargo integration test that drives each `AudioControlRequest` method through
  the request pipeline and asserts the resulting `AudioStatus` broadcasts
- Remove the temporary debug request added in PR 1; confirm tests still pass
- Manual: from the iOS/Android app, send each command via a temporary debug
  button and confirm engine state transitions match expectations

## Phase 3: Preferences cleanup

### PR 8: Honour preferences updates as a device switch

- When a `UpdateRequest { preferences }` arrives and the audio section has
  changed (device, sample rate, channel offsets, buffer size, jack toggle),
  trigger a `stop_audio` -> `start_audio` cycle inside `AudioController`
- Persist preferences with the existing `write_preferences` flow only after the
  new engine has come up (or has failed and reported `Failed`)
- Broadcast `AudioStatus` and the reset `PlaybackState` around the switch

**Verification:**

- Cargo integration test that sends an `UpdateRequest` changing the sample
  rate and asserts the broadcast sequence is
  `AudioStatus(STOPPED) -> PlaybackState(STOPPED) -> AudioStatus(RUNNING)`
- Cargo unit test that asserts preferences are persisted only after the new
  engine reports its state (success or failure)
- Manual: change the device in the iOS preferences UI (PR 10) and confirm the
  switch is seamless and persisted across restarts

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

**Verification:**

- Cargo unit test that confirms a stored preferences file containing a stale
  `output_channel_count` field is parsed without error (ignored unknown field)
- Cargo unit test that clamps an out-of-range `main_channel_offset` against a
  small native channel count
- Manual: confirm `AudioStatus.channel_count` matches the OS-reported channel
  count on macOS (2), a multichannel USB interface (e.g. 8), and Raspberry Pi
  (2)

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

**Verification:**

- XCTest snapshot / view tests for `PreferencesView` that assert the picker
  renders the expected options from a stub `AudioDevices` response
- XCTest that injects a `FAILED` `AudioStatus` and asserts the warning banner
  is visible on both the preferences screen and the transport area
- Manual: change device and sample rate via the picker on a real iPad and
  confirm audio restarts with the new settings; pull-to-refresh updates the
  list when a USB interface is plugged in

### PR 11: Android device selection UI

- Mirror the iOS work in `android/app/src/main/java/com/joenoel/bloop/ui/PreferencesScreen.kt`
  using Jetpack Compose dropdowns (`ExposedDropdownMenuBox`) for device and
  sample rate
- Subscribe to `AudioStatus` via the existing state store and middleware
- Surface the same `FAILED` / `STOPPED` callback warning on the preferences
  screen and on the main transport area
- Add a "Restart audio" action

**Verification:**

- Compose UI tests that assert the dropdowns populate from a stub
  `AudioDevices` and that selecting an option dispatches the expected store
  action
- Compose UI test that injects a `FAILED` `AudioStatus` and asserts the
  warning banner appears on both the preferences screen and the transport area
- Manual: change device on a physical Android device with a USB-C audio
  interface attached and confirm audio restarts with the new settings

### PR 12: Editor device selection UI (optional, follow-up)

- Bring the same picker UX to the desktop editor (`editor/`), reusing the
  shared proto schema and the existing theme tokens
- Show the audio status banner in the editor when the callback is not running

**Verification:**

- React component tests (Jest / React Testing Library) that assert the picker
  renders from a stub `AudioDevices` and that the banner appears when
  `AudioStatus.engine_state` is `FAILED`
- Manual: run `yarn start` in `editor/`, change device and sample rate, and
  confirm the core restarts audio accordingly

## Phase 5: Polish and resilience

### PR 13: Hot-plug device topology updates

- Detect device add/remove events from `cpal` where the platform supports it
  (or poll on a slow interval as a fallback) and rebroadcast `AudioDevices`
- If the currently selected device disappears, transition the engine to
  `Stopped` and broadcast `AudioStatus` so the apps can prompt the user

**Verification:**

- Cargo unit test using a fake device-enumeration source that simulates a
  device disappearing and asserts the engine transitions to `Stopped` and an
  updated `AudioDevices` is broadcast
- Manual: plug and unplug a USB audio interface on macOS, Linux, and Android
  while the app is running; confirm the device list updates and that removing
  the active device stops audio with a visible UI warning

### PR 14: Tests and CI coverage

- Promote the unit tests added in earlier PRs into a coherent
  `core/tests/audio_lifecycle.rs` integration suite covering start/stop cycles,
  preference-driven restart, device-disappears-while-running, and
  callback-stall detection (using the dummy backend)
- Add iOS XCUITest and Android instrumented test that assert the warning
  banner appears when `AudioStatus.engine_state` is `FAILED`
- Wire these test targets into the existing CI workflows

**Verification:**

- All new tests pass in CI on every PR
- Manual: review the CI run for one PR after merge and confirm the new audio
  lifecycle suite executes

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
