# Android Port Work Breakdown

This document breaks the Android port into a sequence of small, reviewable PRs.

## Phase 1: Rust Core Foundation

### PR 1: Adjust Rust features for Android

- Remove or isolate the `jack` feature from `cpal` for Android builds
- Ensure Android builds use `--no-default-features` so desktop UI and MIDI can stay off initially
- Keep Linux and iOS builds unchanged

### PR 2: Android runtime path and hostname handling

- Update `core/src/config.rs` so Android relies on `BLOOP_HOME` supplied by the app layer
- Add a safe Android fallback for hostname-dependent service naming in `core/src/network/server.rs`
- Document any Android-specific environment expectations

Android environment expectations for PR 2:

- Android app layer must set `BLOOP_HOME` before initializing core
- If `BLOOP_HOME` is missing on Android, core exits early with a clear startup error
- mDNS service instance naming must remain stable even when hostname lookup fails or returns non-UTF-8

### PR 3: Android Rust build script

- Add a `scripts/build-android.sh` script
- Build at least `aarch64-linux-android`
- Optionally include emulator/device targets such as `x86_64-linux-android` and `armv7-linux-androideabi`
- Stage generated native libraries in a predictable output directory
- Add a `.cargo/config.toml` that sets `CC_aarch64_linux_android` and `AR_aarch64_linux_android` via the NDK toolchain path, so Android builds work without manual shell exports (discovered during PR 2 validation)

## Phase 2: Android App Scaffold

### PR 4: Create the Android app project

- Add a new `android/` app project
- Configure Gradle, Android manifest, and a minimal activity
- Set up Jetpack Compose for the UI layer

### PR 5: Load the Rust library from Android

- Add native build wiring for the Rust output
- Add a JNI bridge that wraps `bloop_init`, `bloop_add_request`, and `bloop_shutdown`
- Add a Kotlin wrapper class for the native core lifecycle

### PR 6: Share Protobuf models with Android

- Generate Kotlin or Java classes from `api/bloop.proto`
- Keep the Android request and response schema aligned with the Rust core and iOS app
- Verify request encoding and response decoding locally

## Phase 3: State, FFI, and Networking

### PR 7: Port the app state architecture

- Recreate the iOS app state shape in Android
- Implement actions, reducers, and a central store using Android-native patterns such as `ViewModel` and `StateFlow`
- Keep request and response flow close to the current iOS structure

### PR 8: Add local-core middleware

- Start and stop the embedded Rust core from Android
- Route raw requests into the FFI bridge
- Route raw responses back into the app store

### PR 9: Add request and response codec middleware

- Encode typed app actions into Protobuf requests
- Decode incoming raw Protobuf responses into typed state updates
- Mirror the current iOS middleware split where practical

### PR 10: Add remote server connectivity

- Implement WebSocket client support for connecting to an external Bloop server
- Match the current remote/local dual-mode behavior from iOS
- Reuse the shared request and response pipeline

### PR 11: Add Android service discovery

- Discover `_bloop._tcp` services on the local network
- Feed discovered servers into app state
- Support restarting discovery from the UI

### PR 12: Add Android audio session handling

- Request and release Android audio focus when the local core starts or stops
- Map the current iOS audio session behavior to Android equivalents

## Phase 4: UI Port

### PR 13: Server selection screen

- Port the server selection flow
- Support local start, remote connect, scanning state, and retry

### PR 14: Transport controls

- Port play, stop, loop, queue, and previous/next song controls
- Port beat and metronome indicators

### PR 15: Sections grid

- Port the section list and selection behavior
- Handle compact and expanded layouts

### PR 16: Song screen

- Port song editing, sample selection, and section navigation
- Support sample import for `.wav` files

### PR 17: Main project screen

- Port the top-level project navigation and toolbar structure
- Include account actions and modal entry points

### PR 18: Projects and sync UI

- Port project selection and project sync status views
- Show local and cloud project lists

### PR 19: Preferences UI

- Port audio and MIDI settings first
- Omit or defer Raspberry Pi switch mapping UI if it is not relevant on Android

### PR 20: Waveform rendering

- Port waveform display for sample feedback and playback context

## Phase 5: Product Completion

### PR 21: Sample upload flow

- Port sample upload behavior to Android
- Keep compatibility with the existing backend API

### PR 22: Authentication flow

- Port login and logout behavior
- Reconnect auth state to project and sync features

### PR 23: Device behavior polish

- Prevent screen sleep during active use
- Handle lifecycle and background transitions safely

### PR 24: Android CI

- Add CI coverage for Android app builds
- Add Rust Android build validation to the project workflow where appropriate
- Run Android build and unit tests on every pull request and on pushes to `main`
- On release publication, build Android and attach a debug APK to the GitHub Release for local device install

### PR 25: Android release signing

- Add release signing configuration for Android builds
- Store keystore and signing credentials in GitHub secrets
- Build and publish signed release APK (and optionally AAB for Play Store)

## Suggested Order

1. PRs 1-3: Rust and build groundwork
2. PRs 4-6: Android project and native integration
3. PRs 7-12: State, FFI, networking, and discovery
4. PRs 13-20: UI port
5. PRs 21-25: Completion and CI

## Parallel Work Opportunities

- PR 3 can proceed alongside PR 4
- PR 6 can proceed once the Android project exists
- PRs 13-20 can be split across multiple contributors after PR 7 establishes shared app state and after the integration layer is stable
