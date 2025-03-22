import SwiftUI

struct SectionView: View {
    var section: Bloop_Section
    var selections: Bloop_Selections
    var playbackState: Bloop_PlaybackState
    var progress: Bloop_Progress
    var dispatch: Dispatch

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
                Text(section.name)
                    .font(.title2)

                Spacer()

                statusIcons
            }
            .padding([.leading, .trailing])

        }
        .frame(minHeight: 48)
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

struct SectionView_Previews: PreviewProvider {
    static let section = {
        let section = demoSection(0)
        return section
    }()

    static let selections = {
        Bloop_Selections.with {
            $0.section = section.id
        }
    }()

    static let playbackState = {
        Bloop_PlaybackState.with {
            $0.sectionID = section.id
        }
    }()

    static let progress = {
        Bloop_Progress.with {
            $0.sectionProgress = 0.5
        }
    }()

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
