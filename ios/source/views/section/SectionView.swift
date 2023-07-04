import SwiftUI

struct SectionView: View {
    var section: Section
    var selections: Selections
    var playbackState: PlaybackState
    var progress: Progress
    var dispatch: Dispatch
    @State private var editing = false

    private var isSelected: Bool {
        selections.section == section.id
    }

    private var isPlaying: Bool {
        playbackState.sectionId == section.id
    }

    private var border: some View {
        let borderColour =
            isPlaying ? Colours.playing : isSelected ? Colours.selected : Colours.neutral6

        return Rectangle()
            .frame(width: 2)
            .foregroundColor(borderColour)
    }

    private var editButton: some View {
        Button {
            editing = true
        } label: {
            Label("Edit", systemImage: "pencil")
                .background(.regularMaterial)
        }
        .buttonStyle(.bordered)
        .popover(isPresented: $editing) {
            EditSectionView(section: section, dispatch: dispatch)
                .presentationDetents([.medium])
        }
    }

    private var statusIcons: some View {
        HStack {
            if section.loop {
                Image(systemName: "repeat")
            }

            if section.metronome {
                Image(systemName: "metronome.fill")
            }
        }
    }

    var body: some View {
        VStack(spacing: Layout.units(0.5)) {
            HStack {
                Text(section.name)
                    .font(.title2)

                Spacer()

                statusIcons

                if isSelected {
                    editButton
                }
            }
            .padding([.leading])

            if isPlaying {
                ProgressBar(progress: progress.sectionProgress)
                    .frame(maxHeight: 2)
                    .foregroundColor(Colours.playing)
            }
            else {
                Spacer()
                    .frame(height: 2)
            }
        }

        .contentShape(Rectangle())
        .overlay(border, alignment: .leading)
        .frame(minHeight: Layout.touchTarget)
        .onTapGesture {
            if !isSelected {
                let action = selectSectionAction(section.id)
                dispatch(action)
            }
        }

    }
}

struct SectionView_Previews: PreviewProvider {
    static let section = {
        let section = demoSection(0)
        return section
    }()

    static let selections = {
        var selections = Selections()
        selections.section = section.id
        return selections
    }()

    static let playbackState = {
        var playbackState = PlaybackState()
        playbackState.sectionId = section.id
        return playbackState
    }()

    static let progress = Progress()

    static var previews: some View {
        SectionView(
            section: section,
            selections: selections,
            playbackState: playbackState,
            progress: progress
        ) {
            action in
            print("Dipatch: \(action)")
        }
        .padding()
    }
}
