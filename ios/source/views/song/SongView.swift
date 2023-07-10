import SwiftUI

struct SongView: View {
    var song: Song
    var selections: Selections
    var playbackState: PlaybackState
    var progress: Progress
    var waveforms: Waveforms
    var dispatch: Dispatch

    @State private var editingName = false
    @State private var editingSections = false
    @State private var editingSample = false
    @State private var editingTempo = false

    @State private var newName: String = ""
    @State private var newTempo: Double = 120.0

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

                    if song.sample == nil {
                        Button {
                            editingSample = true
                        } label: {
                            Label("Add Sample", systemImage: "waveform")
                        }
                    }
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
        .overlay(alignment: .leading) { sidebar }
        .overlay(alignment: .trailing) { sidebar }
        .onTapGesture {
            if !isSelected {
                selectSong()
            }

        }
        .background(.thickMaterial)
        .cornerRadius(Layout.cornerRadiusLarge)
        .shadow(radius: 4.0)
        .sheet(isPresented: $editingSections) {
            SectionsView(song: song, dispatch: dispatch)
        }
        .tint(waveformColour)
        .fileImporter(isPresented: $editingSample, allowedContentTypes: [.wav]) { result in
            switch result {
            case .success(let url):
                let action = Action.uploadSample((song.id, url))
                dispatch(action)

            case .failure(let error):
                print("\(error)")
            }
        }
    }

    @ViewBuilder
    var sidebar: some View {
        if isPlaying {
            Colours.playing
                .frame(width: Layout.units(1))
        }
        else if isSelected {
            Colours.selected
                .frame(width: Layout.units(1))
        }
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
                        Label("Rename", systemImage: "pencil")
                    }

                    Button {
                        editingSections = true
                    } label: {
                        Label("Sections", systemImage: "rectangle.grid.1x2")
                    }

                    Button {
                        editingSample = true
                    } label: {
                        Label(
                            song.sample == nil ? "Add Sample" : "Replace Sample",
                            systemImage: "waveform"
                        )
                    }

                    Button {
                        editingTempo = true
                    } label: {
                        Label("Tempo", systemImage: "metronome")
                    }

                    Button(role: .destructive) {
                        let action = removeSongAction(song.id)
                        dispatch(action)
                    } label: {
                        Label("Remove", systemImage: "trash")
                    }

                } label: {
                    Image(systemName: "ellipsis")
                }
                .font(.title)
            }

        }
        .popover(isPresented: $editingName) {
            NameEditor(prompt: "Song Name", value: $newName)
                .onSubmit {
                    var song = song
                    song.name = newName

                    let action = updateSongAction(song)
                    dispatch(action)

                    editingName = false
                }
                .onAppear {
                    newName = song.name
                }
        }
        .popover(isPresented: $editingTempo) {
            VStack(alignment: .leading, spacing: Layout.units(2)) {
                Text("Tempo")

                TextField("Tempo", value: $newTempo, formatter: NumberFormatter())
                    .textFieldStyle(.roundedBorder)
            }
            .font(.title2)
            .padding(Layout.units(2))
            .frame(minWidth: 400)
            .background(.regularMaterial)
            .onAppear {
                if let tempo = song.sample?.tempo.bpm {
                    self.newTempo = tempo
                }
            }
            .onSubmit {
                var song = song
                song.sample?.tempo.bpm = newTempo
                song.tempo.bpm = newTempo

                let action = updateSongAction(song)
                dispatch(action)
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
