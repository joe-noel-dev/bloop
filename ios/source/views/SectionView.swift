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
                    .fontWeight(.semibold)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .foregroundColor(.primary)

                Spacer()

                statusIcons

            }
            .padding([.leading, .trailing])

        }
        .frame(minHeight: 64)
        .contentShape(Rectangle())
        .overlay(alignment: .leading) {
            border
        }
        
        .background(alignment: .leading) {

            // Progress overlay (only when playing)
            if isPlaying {
                GeometryReader { geometry in
                    Rectangle()
                        .fill(Colours.playing.opacity(0.3))
                        .frame(width: progress.sectionProgress * geometry.size.width)
                }
            }
        }
        .background(isQueued ? .thickMaterial : .thinMaterial)
        .simultaneousGesture(TapGesture().onEnded {
            if !isSelected {
                let action = selectSectionAction(section.id)
                dispatch(action)
            }
        })
        .cornerRadius(Layout.cornerRadiusSmall)
    }
}

struct SectionView_Previews: PreviewProvider {

    struct PreviewWrapper: View {
        var section = demoSection(0)

        var body: some View {
            SectionView(
                section: section,
                selections: selections,
                playbackState: playbackState,
                progress: progress,
                dispatch: loggingDispatch
            )
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
