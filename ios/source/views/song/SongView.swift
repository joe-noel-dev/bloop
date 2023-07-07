import SwiftUI

struct SongView: View {
    var song: Song
    var selections: Selections
    var playbackState: PlaybackState
    var progress: Progress
    var waveforms: Waveforms
    var dispatch: Dispatch

    @State private var editingName = false
    @State private var newName: String = ""

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

    init(
        song: Song,
        selections: Selections,
        playbackState: PlaybackState,
        progress: Progress,
        waveforms: Waveforms,
        dispatch: @escaping Dispatch
    ) {
        self.song = song
        self.selections = selections
        self.playbackState = playbackState
        self.progress = progress
        self.waveforms = waveforms
        self.dispatch = dispatch
        self.newName = song.name
    }

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
        #if os(iOS)
            let columnCount = horizontalSizeClass == .compact ? 1 : 2
        #else
            let columnCount = 2
        #endif

        return Array(repeating: GridItem(.flexible()), count: columnCount)
    }

    private var sampleId: Id? {
        song.sample?.id
    }

    private var waveformData: WaveformData? {
        guard let sampleId = sampleId else {
            return nil
        }

        return waveforms[sampleId]
    }
    
    private var waveformColour: Color {
        if isPlaying {
            return Colours.playing
        }
        
        if isSelected {
            return Colours.selected
        }
        
        return Colours.neutral4
    }

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                header

                ZStack {
                    WaveformView(waveform: waveformData)
                }
                .frame(height: 120)
                .foregroundColor(waveformColour)
                
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
                    .frame(width: Layout.units(1))
            }
            else if isSelected {
                Colours.selected
                    .frame(width: Layout.units(1))
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

                    Button {
                        editingName = true
                    } label: {
                        Text("Rename")
                    }

                    Button(role: .destructive) {
                        let action = removeSongAction(song.id)
                        dispatch(action)
                    } label: {
                        Text("Remove Song")
                    }

                } label: {
                    Image(systemName: "ellipsis")
                }
                .font(.title)
            }

        }
        .popover(isPresented: $editingName) {
            NameEditor(value: $newName)
                .onSubmit {
                    var song = song
                    song.name = newName

                    let action = updateSongAction(song)
                    dispatch(action)

                    editingName = false
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
            waveforms: Waveforms(),
            dispatch: loggingDispatch
        )
        .padding()

    }
}
