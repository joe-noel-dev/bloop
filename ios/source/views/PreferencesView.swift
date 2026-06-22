import SwiftUI

struct PreferencesView: View {
    var preferences: Bloop_Preferences?
    var audioDevices: Bloop_AudioDevices?
    var audioStatus: Bloop_AudioStatus?
    var dispatch: Dispatch
    var onDismiss: () -> Void

    @State private var editedPreferences: Bloop_Preferences
    @State private var showingSaveConfirmation = false
    @State private var isSaving = false

    init(
        preferences: Bloop_Preferences?,
        audioDevices: Bloop_AudioDevices?,
        audioStatus: Bloop_AudioStatus?,
        dispatch: @escaping Dispatch,
        onDismiss: @escaping () -> Void
    ) {
        self.preferences = preferences
        self.audioDevices = audioDevices
        self.audioStatus = audioStatus
        self.dispatch = dispatch
        self.onDismiss = onDismiss
        self._editedPreferences = State(initialValue: preferences ?? Bloop_Preferences())
    }

    var body: some View {
        NavigationStack {
            Form {
                audioStatusSection
                audioSection
                midiSection
                if editedPreferences.switchAvailable {
                    switchSection
                }
            }
            .navigationTitle("Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        onDismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Save") {
                        isSaving = true
                        dispatch(updatePreferencesAction(editedPreferences))
                    }
                    .disabled(isSaving)
                }
            }
            .onAppear {
                dispatch(getPreferencesAction())
                dispatch(getAudioDevicesAction())
            }
            .onChange(of: preferences) { oldValue, newValue in
                if let newValue = newValue {
                    editedPreferences = newValue
                    if isSaving {
                        showingSaveConfirmation = true
                        isSaving = false
                    }
                }
            }
            .refreshable {
                dispatch(getAudioDevicesAction())
                dispatch(getPreferencesAction())
            }
            .alert("Saved", isPresented: $showingSaveConfirmation) {
                Button("OK") {
                    onDismiss()
                }
            } message: {
                Text("Preferences have been saved successfully.")
            }
        }
    }

    @ViewBuilder
    private var audioStatusSection: some View {
        if let status = audioStatus {
            let isNotRunning = status.engineStatus != .audioEngineRunning
            if isNotRunning {
                Section {
                    HStack(spacing: Layout.units(1.5)) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundColor(.yellow)
                        VStack(alignment: .leading, spacing: 2) {
                            Text(status.engineStatus == .audioEngineFailed ? "Audio engine failed" : "Audio engine stopped")
                                .font(.subheadline)
                                .fontWeight(.semibold)
                            if !status.error.isEmpty {
                                Text(status.error)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                        Spacer()
                        Button("Restart") {
                            dispatch(audioControlAction(method: .audioControlRestart))
                        }
                        .buttonStyle(.borderedProminent)
                        .controlSize(.small)
                    }
                    .padding(.vertical, 4)
                }
            } else {
                Section(header: Text("Audio Status")) {
                    LabeledContent("Device", value: status.currentDeviceName)
                    LabeledContent("Sample Rate", value: "\(status.currentSampleRate) Hz")
                    LabeledContent("Channels", value: "\(status.currentChannelCount)")
                }
            }
        }
    }

    private var selectedDevice: Bloop_AudioDevice? {
        audioDevices?.devices.first { $0.id == editedPreferences.audio.outputDevice }
    }

    private var availableSampleRates: [UInt32] {
        selectedDevice?.supportedSampleRates.sorted() ?? []
    }

    @ViewBuilder
    private var audioSection: some View {
        Section(header: Text("Audio")) {
            if let devices = audioDevices, !devices.devices.isEmpty {
                Picker("Output Device", selection: Binding(
                    get: { editedPreferences.audio.outputDevice },
                    set: { newId in
                        editedPreferences.audio.outputDevice = newId
                        if let device = devices.devices.first(where: { $0.id == newId }),
                           !device.supportedSampleRates.isEmpty,
                           !device.supportedSampleRates.contains(editedPreferences.audio.sampleRate) {
                            editedPreferences.audio.sampleRate = device.supportedSampleRates.min() ?? editedPreferences.audio.sampleRate
                        }
                    }
                )) {
                    ForEach(devices.devices, id: \.id) { device in
                        Text(device.name).tag(device.id)
                    }
                }
                .pickerStyle(.menu)
            } else {
                TextField("Output Device", text: Binding(
                    get: { editedPreferences.audio.outputDevice },
                    set: { editedPreferences.audio.outputDevice = $0 }
                ))
            }

            if !availableSampleRates.isEmpty {
                Picker("Sample Rate", selection: Binding(
                    get: { editedPreferences.audio.sampleRate },
                    set: { editedPreferences.audio.sampleRate = $0 }
                )) {
                    ForEach(availableSampleRates, id: \.self) { rate in
                        Text("\(rate) Hz").tag(rate)
                    }
                }
                .pickerStyle(.menu)
            } else {
                HStack {
                    Text("Sample Rate")
                    Spacer()
                    TextField("Sample Rate", value: Binding(
                        get: { editedPreferences.audio.sampleRate },
                        set: { editedPreferences.audio.sampleRate = $0 }
                    ), format: .number)
                    .multilineTextAlignment(.trailing)
                    .keyboardType(.numberPad)
                }
            }

            HStack {
                Text("Buffer Size")
                Spacer()
                TextField("Buffer Size", value: Binding(
                    get: { editedPreferences.audio.bufferSize },
                    set: { editedPreferences.audio.bufferSize = $0 }
                ), format: .number)
                .multilineTextAlignment(.trailing)
                .keyboardType(.numberPad)
            }

            Toggle("Use Jack", isOn: Binding(
                get: { editedPreferences.audio.useJack },
                set: { editedPreferences.audio.useJack = $0 }
            ))

            HStack {
                Text("Main Channel Offset")
                Spacer()
                TextField("Main Offset", value: Binding(
                    get: { editedPreferences.audio.mainChannelOffset },
                    set: { editedPreferences.audio.mainChannelOffset = $0 }
                ), format: .number)
                .multilineTextAlignment(.trailing)
                .keyboardType(.numberPad)
            }

            HStack {
                Text("Click Channel Offset")
                Spacer()
                TextField("Click Offset", value: Binding(
                    get: { editedPreferences.audio.clickChannelOffset },
                    set: { editedPreferences.audio.clickChannelOffset = $0 }
                ), format: .number)
                .multilineTextAlignment(.trailing)
                .keyboardType(.numberPad)
            }
        }
    }

    @ViewBuilder
    private var midiSection: some View {
        Section(header: Text("MIDI")) {
            TextField("Input Device", text: Binding(
                get: { editedPreferences.midi.inputDevice },
                set: { editedPreferences.midi.inputDevice = $0 }
            ))
        }
    }

    @ViewBuilder
    private var switchSection: some View {
        Section(
            header: Text("Switches"),
            footer: Button(action: addSwitchMapping) {
                Label("Add Mapping", systemImage: "plus.circle.fill")
            }
        ) {
            if editedPreferences.switch.mappings.isEmpty {
                Text("No switch mappings configured")
                    .foregroundColor(.secondary)
            } else {
                ForEach(editedPreferences.switch.mappings.indices, id: \.self) { index in
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Text("Pin")
                            Spacer()
                            TextField("Pin", value: Binding(
                                get: { editedPreferences.switch.mappings[index].pin },
                                set: { editedPreferences.switch.mappings[index].pin = $0 }
                            ), format: .number)
                            .multilineTextAlignment(.trailing)
                            .keyboardType(.numberPad)
                            .frame(width: 100)
                        }

                        HStack {
                            Text("Gesture")
                            Spacer()
                            Picker("", selection: Binding(
                                get: { editedPreferences.switch.mappings[index].gesture },
                                set: { editedPreferences.switch.mappings[index].gesture = $0 }
                            )) {
                                ForEach(allGestures, id: \.self) { gesture in
                                    Text(gestureName(gesture)).tag(gesture)
                                }
                            }
                            .pickerStyle(.menu)
                        }

                        HStack {
                            Text("Action")
                            Spacer()
                            Picker("", selection: Binding(
                                get: { editedPreferences.switch.mappings[index].action },
                                set: { editedPreferences.switch.mappings[index].action = $0 }
                            )) {
                                ForEach(allActions, id: \.self) { action in
                                    Text(actionName(action)).tag(action)
                                }
                            }
                            .pickerStyle(.menu)
                        }
                    }
                    .padding(.vertical, 4)
                }
                .onDelete(perform: deleteSwitchMapping)
            }
        }
    }

    private func gestureName(_ gesture: Bloop_Gesture) -> String {
        switch gesture {
        case .press: return "Press"
        case .release: return "Release"
        case .hold: return "Hold"
        default: return "Unknown"
        }
    }

    private func actionName(_ action: Bloop_Action) -> String {
        switch action {
        case .previousSong: return "Previous Song"
        case .nextSong: return "Next Song"
        case .previousSection: return "Previous Section"
        case .nextSection: return "Next Section"
        case .queueSelected: return "Queue Selected"
        case .toggleLoop: return "Toggle Loop"
        case .togglePlay: return "Toggle Play"
        default: return "Unknown"
        }
    }

    private var allGestures: [Bloop_Gesture] {
        [.press, .release, .hold]
    }

    private var allActions: [Bloop_Action] {
        [
            .previousSong, .nextSong, .previousSection, .nextSection,
            .queueSelected, .toggleLoop, .togglePlay,
        ]
    }

    private func addSwitchMapping() {
        let newMapping = Bloop_SwitchMapping.with {
            $0.pin = 0
            $0.gesture = .press
            $0.action = .togglePlay
        }
        editedPreferences.switch.mappings.append(newMapping)
    }

    private func deleteSwitchMapping(at offsets: IndexSet) {
        editedPreferences.switch.mappings.remove(atOffsets: offsets)
    }
}
