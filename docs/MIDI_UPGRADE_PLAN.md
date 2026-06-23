# MIDI Upgrade Plan

This document breaks the MIDI improvements into a sequence of small, reviewable
PRs. The goals are:

- Hot-plug support: connect MIDI devices after the app has started
- Preferences change from a single device name to a list of enabled devices
- The preferences UI shows all discovered MIDI devices and lets the user toggle
  each one on/off
- A file-based mapping system where each device's mappings live in their own
  file, with the existing iCON G_Boar mappings as the first example

## Current state (for context)

- `core/src/midi/controller.rs` owns `MidiController`, constructed once at
  startup in `core/src/control/main.rs`. It opens a single port whose name is
  matched against `preferences.input_device` (defaulting to
  `"iCON G_Boar V1.03"`) and holds a single `MidiInputConnection<Context>`.
- `core/src/midi/matcher.rs` defines the `Matcher` trait and `ExactMatcher`.
  Mappings are hardcoded in `get_mappings()` inside `controller.rs`.
- `MidiPreferences` in `api/bloop.proto` has one field: `string input_device`.
- The iOS preferences UI (`PreferencesView.swift`) and Android UI
  (`PreferencesScreen.kt`) both show a single free-text "Input Device" field.
- The `midir` crate (v0.11) is the MIDI backend, gated behind the `midi`
  Cargo feature. There is no hot-plug mechanism.

---

## Phase 1: File-based device mappings

### PR 1: Extract device mappings into separate files

**Goal:** Move the hardcoded `get_mappings()` out of `controller.rs` and into a
dedicated mapping file per device, loaded at startup from a well-known directory
inside the Bloop home folder.

**Changes:**

- Define a `MidiDeviceMapping` struct:
  ```
  device_regex   – regex pattern that matches a MIDI port name
  mappings       – Vec<Mapping> (message bytes → Action)
  ```
- Create `core/src/midi/mappings/` directory in the source tree for
  compiled-in default mappings. Add
  `core/src/midi/mappings/icon_g_boar.rs` that declares the seven existing
  iCON G_Boar mappings and the regex `"iCON G_Boar"`.
- Load user-supplied mapping files (TOML or JSON) from
  `$BLOOP_HOME/midi_mappings/` at startup, merging them with the compiled-in
  defaults. Unknown files are skipped with a warning.
- `controller.rs` calls a `load_mappings()` function that returns
  `Vec<MidiDeviceMapping>` and uses the regex to decide which mappings apply
  to each connected port.

**Verification:**

- Cargo unit tests that assert `icon_g_boar.rs` produces seven mappings and
  the regex matches `"iCON G_Boar V1.03"` but not `"Generic MIDI"`.
- Cargo unit test for `load_mappings()` that reads a temporary directory
  containing one well-formed file and one malformed file and asserts only the
  valid mappings are returned.
- Manual: run Bloop on macOS/Linux with the iCON G_Boar connected; confirm
  all seven mappings still work.

---

## Phase 2: Multi-device preferences

### PR 2: Change `MidiPreferences` to a list of enabled device patterns

**Goal:** Replace the single `input_device` string with a list of device name
patterns the user wants active.

**Changes:**

- In `api/bloop.proto`, change `MidiPreferences` from:
  ```protobuf
  message MidiPreferences {
      string input_device = 1;
  }
  ```
  to:
  ```protobuf
  message MidiPreferences {
      repeated string enabled_devices = 1;
  }
  ```
  `enabled_devices` is a list of substrings/regexes; a port is enabled if its
  name matches any entry.
- Regenerate proto bindings on Rust, Swift, and Kotlin sides.
- Update `controller.rs` to iterate over all available ports and open a
  connection on every port whose name matches any pattern in `enabled_devices`.
  Store `Vec<MidiInputConnection<Context>>` instead of `Option<...>`.
- Update `default_midi_preferences()` in `preferences/mod.rs` to default
  `enabled_devices` to `["iCON G_Boar"]` so existing users are not broken.
- Update `validate_preferences` if needed.
- Write a migration note: an existing `preferences.json` with
  `"inputDevice": "..."` will be silently ignored (unknown field); users will
  need to re-add their device via the UI.

**Verification:**

- Cargo unit test: build `MidiPreferences` with two patterns, create a stub
  port list with three names, and assert the correct subset is opened.
- Cargo unit test: confirm `read_preferences_from_str` with the old
  `"inputDevice"` key parses without error and results in an empty
  `enabled_devices` list (unknown-field behaviour already present via
  `ignore_unknown_fields: true`).
- `cargo build` succeeds; Xcode build and Gradle build succeed against
  regenerated bindings.
- Manual: add two patterns, connect two matching devices, confirm both deliver
  actions.

---

## Phase 3: MIDI device discovery API

### PR 3: Expose available MIDI devices via the proto API

**Goal:** Let the UI query the core for a list of currently attached MIDI input
port names so the preferences screen can render real checkboxes instead of
free-text fields.

**Changes:**

- Add to `api/bloop.proto`:
  ```protobuf
  message MidiDevices {
      repeated string port_names = 1;
  }
  ```
- Add `MIDI_DEVICES` to the `Entity` enum.
- Wire `MIDI_DEVICES` into `Response` and handle a `Get { MIDI_DEVICES }`
  request in `control/main.rs` by enumerating live ports with `midir` and
  returning the list.
- Add a `get_midi_devices()` helper in `controller.rs` (or a small
  `midi/devices.rs` module) that creates a short-lived `MidiInput`, queries
  ports, collects names, and returns them without opening any connections.
- Regenerate proto bindings.

**Verification:**

- Cargo integration test: dispatch `Get { MIDI_DEVICES }` and assert the
  response contains a `MidiDevices` message (may be empty on CI, gated with
  the `midi` feature flag).
- Round-trip serialization unit test for `MidiDevices`.
- Manual: query from the iOS/Android app (debug log or temporary UI) with
  and without a device connected; confirm the list updates.

---

## Phase 4: Hot-plug support in the core

### PR 4: Poll for MIDI port changes and reconnect automatically

**Goal:** Automatically connect to newly attached devices (and disconnect from
removed ones) without restarting the app.

**Changes:**

- Replace the one-shot connection in `MidiController::new` with a background
  task (Tokio task or a `std::thread`) that:
  1. Polls the available port list every ~2 seconds (or uses `midir`'s
     `virtual_ports` if the platform supports callbacks — fall back to polling
     universally for now).
  2. Diffs the previous list against the current list.
  3. For any new port whose name matches `enabled_devices`, opens a connection
     and stores it.
  4. For any removed port, drops the corresponding connection.
  5. Broadcasts an updated `MidiDevices` message whenever the list changes.
- Hold `MidiController` in the `Arc<Mutex<...>>` pattern already used by
  `AudioController` so the background task can update the connections.
- On preferences update (`UpdateRequest { preferences.midi }`) re-run the
  matching logic against all currently known ports.

**Verification:**

- Cargo unit test using a fake port-enumeration source (dependency-injected
  function) that simulates a device appearing, confirms a connection attempt
  is made, then simulates removal and confirms the connection is dropped.
- Cargo unit test that sends an `UpdateRequest` with changed `enabled_devices`
  and asserts the controller reconnects to the now-matching ports.
- Manual: start Bloop, plug in the iCON G_Boar (or any mapped device), wait
  ~5 s, confirm MIDI input works without a restart; unplug and replug, confirm
  it reconnects.

---

## Phase 5: Preferences UI

### PR 5: iOS MIDI preferences UI

**Goal:** Replace the free-text "Input Device" field with a list of discovered
ports, each with a toggle to enable/disable it.

**Changes:**

- In `PreferencesView.swift`, when the view appears dispatch
  `Get { MIDI_DEVICES }` alongside the existing `Get { PREFERENCES }` and
  `Get { AUDIO_DEVICES }`.
- Pass `MidiDevices` down to `midiSection`.
- Render a `List` (or `ForEach` inside the existing `Form`) showing every
  discovered port name. Next to each, a `Toggle` bound to whether that port
  name is in `editedPreferences.midi.enabledDevices`. Toggling updates
  `editedPreferences` in the usual pattern; the user still hits "Save" to
  commit.
- Show a "No MIDI devices found" placeholder when the list is empty.
- Pull-to-refresh re-fetches `MIDI_DEVICES`.

**Verification:**

- XCTest view test that injects a stub `MidiDevices` with two port names and
  asserts two toggle rows are rendered.
- XCTest that toggles one row on, hits Save, and asserts the dispatched
  `UpdateRequest` contains the expected `enabled_devices` list.
- Manual: open preferences with a device attached; confirm it appears as a
  toggle; enable it; save; confirm it persists across app relaunch.

### PR 6: Android MIDI preferences UI

**Goal:** Mirror the iOS work in `PreferencesScreen.kt`.

**Changes:**

- Dispatch `Get { MIDI_DEVICES }` in `LaunchedEffect(Unit)` alongside the
  existing preference and audio requests.
- Store `midiDevices` in `AppState` and surface it through the existing Redux-
  style middleware.
- In the `SectionHeader("MIDI")` block, render an `ExposedDropdownMenuBox`
  **or** a vertical list of `Row`s each with a `Switch`, one per port name.
  The `Switch` state is derived from whether the port name is in
  `edited.midi.enabledDevicesList`; toggling calls `edited.toBuilder()` to add
  or remove the entry.
- Show "No MIDI devices found" when the list is empty.

**Verification:**

- Compose UI test that provides a stub `MidiDevices` and asserts the correct
  number of `Switch` rows are displayed.
- Compose UI test that toggles a row and asserts `edited.midi.enabledDevicesList`
  contains the expected entry.
- Manual: physical Android device with a USB MIDI interface; confirm the device
  appears in the list, toggling enables MIDI actions.

---

## Suggested order

1. **PR 1** — File-based mappings (pure Rust, no API changes, self-contained)
2. **PR 2** — Multi-device preferences (proto change, Rust + Swift + Kotlin)
3. **PR 3** — MIDI device discovery API (proto change + Rust plumbing)
4. **PR 4** — Hot-plug in the core (Rust only, depends on PRs 2 & 3)
5. **PR 5 + PR 6** — iOS and Android UI (can be done in parallel, depends on
   PRs 2 & 3)

## Parallel work opportunities

- PRs 5 and 6 (iOS and Android UI) can be authored simultaneously once PRs 2
  and 3 are merged.
- PR 1 (file-based mappings) is fully independent and can be merged first
  without waiting for the proto changes.
- PR 4 (hot-plug) can be developed against the same branch as PRs 2 & 3 once
  the proto shape is agreed.
