import SwiftUI

struct SongView: View {
    var song: Song
    var selections: Selections
    var playbackState: PlaybackState
    var progress: Progress
    var dispatch: Dispatch
    @Environment(\.horizontalSizeClass) var horizontalSizeClass

    private var selectedSection: Section? {
        song.sections.first {
            $0.id == selections.section
        }
    }

    private var isSelected: Bool {
        selections.song == song.id
    }

    private var isPlaying: Bool {
        playbackState.songId == song.id
    }

    private func selectSong() {
        let action = selectSongAction(song.id)
        dispatch(action)
    }

    private var sectionColumns: [GridItem] {
        let columnCount = horizontalSizeClass == .compact ? 1 : 2

        return Array(repeating: GridItem(.flexible()), count: columnCount)
    }

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                header

                WaveformView()
                    .frame(height: 120)

                LazyVGrid(columns: sectionColumns) {
                    ForEach(song.sections) { section in
                        SectionView(
                            section: section,
                            selections: selections,
                            playbackState: playbackState,
                            progress: progress,
                            dispatch: dispatch
                        )
                    }
                }

            }

            Spacer()
        }
        .contentShape(Rectangle())
        .padding()
        .overlay(alignment: .leading) {
            if isPlaying {
                Colours.playing
                    .frame(width: Layout.units(0.5))
            }
            else if isSelected {
                Colours.selected
                    .frame(width: Layout.units(0.5))
            }

        }
        .onTapGesture {
            if !isSelected {
                selectSong()
            }

        }
        .background(.thickMaterial)
        .cornerRadius(Layout.cornerRadiusLarge)
        .shadow(radius: 4.0)
    }

    @ViewBuilder
    var header: some View {
        HStack {
            Text(song.name)
                .font(.largeTitle)
            
            Spacer()
            
            if isSelected {
                Menu {
                    Button(role: .destructive) {
                        let action = removeSongAction(song.id)
                        dispatch(action)
                    } label: {
                        Text("Remove Song")
                    }
                } label: {
                    Image(systemName: "ellipsis")
                }
            }
            
        }

    }
}

struct SongView_Previews: PreviewProvider {
    static let song: Song = {
        var song = demoSong(0)
        song.name = "My Song Name"
        return song
    }()

    static let selections: Selections = {
        .init(song: song.id, section: song.sections[0].id)
    }()

    static let playbackState = {
        var playbackState = PlaybackState()
        playbackState.songId = song.id
        return playbackState
    }()

    static let progress = Progress()

    static var previews: some View {
        SongView(
            song: song,
            selections: selections,
            playbackState: playbackState,
            progress: progress,
            dispatch: loggingDispatch
        )
        .padding()

    }
}
