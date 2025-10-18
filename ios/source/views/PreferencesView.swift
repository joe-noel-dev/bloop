import SwiftUI

struct PreferencesView: View {
    var preferences: Bloop_Preferences?
    var dispatch: Dispatch
    var onDismiss: () -> Void

    @State private var editedPreferences: Bloop_Preferences
    @State private var showingSaveConfirmation = false
    @State private var isSaving = false

    init(preferences: Bloop_Preferences?, dispatch: @escaping Dispatch, onDismiss: @escaping () -> Void) {
        self.preferences = preferences
        self.dispatch = dispatch
        self.onDismiss = onDismiss
        self._editedPreferences = State(initialValue: preferences ?? Bloop_Preferences())
    }

    var body: some View {
        NavigationStack {
            Form {
                audioSection
                midiSection
                switchSection
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
    private var audioSection: some View {
        Section(header: Text("Audio")) {
            TextField("Output Device", text: Binding(
                get: { editedPreferences.audio.outputDevice },
                set: { editedPreferences.audio.outputDevice = $0 }
            ))

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

            HStack {
                Text("Output Channel Count")
                Spacer()
                TextField("Channel Count", value: Binding(
                    get: { editedPreferences.audio.outputChannelCount },
                    set: { editedPreferences.audio.outputChannelCount = $0 }
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
        Section(header: Text("Switches")) {
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
                            Text(gestureName(editedPreferences.switch.mappings[index].gesture))
                                .foregroundColor(.secondary)
                        }

                        HStack {
                            Text("Action")
                            Spacer()
                            Text(actionName(editedPreferences.switch.mappings[index].action))
                                .foregroundColor(.secondary)
                        }
                    }
                    .padding(.vertical, 4)
                }
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
}
