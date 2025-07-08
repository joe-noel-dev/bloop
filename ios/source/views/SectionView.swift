import SwiftUI

struct SectionView: View {
    @Binding var section: Bloop_Section
    var selections: Bloop_Selections
    var playbackState: Bloop_PlaybackState
    var progress: Bloop_Progress
    var dispatch: Dispatch

    @Environment(\.editMode) var editMode

    private var isSelected: Bool {
        selections.section == section.id
    }

    private var isPlaying: Bool {
        playbackState.sectionID == section.id
    }

    private var isQueued: Bool {
        playbackState.queuedSectionID == section.id
    }

    private var border: some View {
        let borderColour =
            isPlaying ? Colours.playing : isSelected ? Colours.selected : Colours.neutral6

        return Rectangle()
            .frame(width: Layout.units(0.5))
            .foregroundColor(borderColour)
    }

    private var statusIcons: some View {
        HStack {
            if section.loop {
                Image(systemName: "repeat")
            }

            if section.metronome {
                Image(systemName: "metronome")
            }
        }
    }

    var body: some View {
        VStack(spacing: Layout.units(0.5)) {
            HStack {
                TextField("Name", text: $section.name)
                    .font(.title2)
                    .fontWeight(.semibold)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .disabled(editMode?.wrappedValue != .active)
                    .textFieldStyle(.plain)
                    .foregroundColor(isPlaying ? Colours.playing : .primary)

                Spacer()

                if editMode?.wrappedValue == .active {
                    SectionEditView(section: $section)
                }
                else {
                    statusIcons
                }

            }
            .padding([.leading, .trailing])

        }
        .frame(minHeight: 64)
        .contentShape(Rectangle())
        .overlay(alignment: .leading) {
            border
        }
        .overlay(alignment: .bottom) {
            if isPlaying {
                ProgressBar(progress: progress.sectionProgress)
                    .frame(maxHeight: 2)
                    .foregroundColor(Colours.playing)
            }

        }
        .background(isQueued ? Material.thickMaterial : Material.thinMaterial)
        .onTapGesture {
            if !isSelected {
                let action = selectSectionAction(section.id)
                dispatch(action)
            }
        }
        .cornerRadius(Layout.cornerRadiusSmall)
    }
}

private struct SectionEditView: View {
    @Binding var section: Bloop_Section

    var body: some View {
        Group {
            TextField("Start", value: $section.start, formatter: NumberFormatter())
                .textFieldStyle(.roundedBorder)
                .frame(maxWidth: 64)
                #if os(iOS)
                    .keyboardType(.decimalPad)
                #endif
                .submitLabel(.return)

            Toggle(
                isOn: $section.metronome,
                label: {
                    Image(systemName: "metronome")
                }
            )
            .toggleStyle(.button)

            Toggle(
                isOn: $section.loop,
                label: {
                    Image(systemName: "repeat")
                }
            )
            .toggleStyle(.button)
        }
    }
}

struct SectionView_Previews: PreviewProvider {

    struct PreviewWrapper: View {
        @State var section = demoSection(0)

        var body: some View {
            SectionView(
                section: $section,
                selections: selections,
                playbackState: playbackState,
                progress: progress
            ) { action in
                print("Dispatch: \(action)")
            }
            .padding()
        }
    }

    static let selections = {
        Bloop_Selections.with {
            $0.section = demoSection(0).id
        }
    }()

    static let playbackState = {
        Bloop_PlaybackState.with {
            $0.sectionID = demoSection(0).id
        }
    }()

    static let progress = {
        Bloop_Progress.with {
            $0.sectionProgress = 0.5
        }
    }()

    static var previews: some View {
        Group {
            PreviewWrapper()

            PreviewWrapper().environment(\.colorScheme, .dark)
        }

    }
}
