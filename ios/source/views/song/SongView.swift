import SwiftUI

struct SongView: View {
    var song: Song
    var songs: [Song]
    var selections: Selections
    var playbackState: PlaybackState
    var progress: Progress
    var waveforms: Waveforms
    var dispatch: Dispatch

    @Binding var navigationPath: [NavigationItem]

    @State private var editingName = false
    @State private var editingSections = false
    @State private var editingSample = false
    @State private var editingTempo = false

    @State private var newName: String = ""
    @State private var newTempo: Double = 120.0

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

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

    @ViewBuilder
    var sections: some View {
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

    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                HStack {
                    if let previousSongName = previousSongName {
                        Button {
                            selectPreviousSong()
                        } label: {
                            Image(systemName: "arrow.left")
                            Text(previousSongName)
                        }
                    }

                    Spacer()

                    if let nextSongName = nextSongName {
                        Button {
                            selectNextSong()
                        } label: {
                            Text(nextSongName)
                            Image(systemName: "arrow.right")
                        }
                    }

                }
                sections
                Spacer()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .contentShape(Rectangle())
        .padding()
        .overlay(alignment: .leading) { sidebar }
        .overlay(alignment: .trailing) { sidebar }
        .onTapGesture {
            if !isSelected {
                selectSong()
            }

        }
        .gesture(
            DragGesture(minimumDistance: 20, coordinateSpace: .global).onEnded { value in
                if value.translation.width < 0 {
                    selectNextSong()
                }

                if value.translation.width > 0 {
                    selectPreviousSong()
                }
            }
        )
        .sheet(isPresented: $editingSections) {
            SectionsView(song: song, dispatch: dispatch)
        }
        .fileImporter(isPresented: $editingSample, allowedContentTypes: [.wav]) { result in
            switch result {
            case .success(let url):
                let action = Action.uploadSample((song.id, url))
                dispatch(action)

            case .failure(let error):
                print("\(error)")
            }
        }
        .toolbar {
            headerMenu
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
        .navigationTitle(song.name)
    }

    private func selectSongWithOffset(_ offset: Int) {
        guard let index = songs.firstIndex(where: { $0.id == song.id }) else {
            return
        }

        let nextIndex = index + offset

        guard 0 <= nextIndex && nextIndex < songs.count else {
            return
        }

        let nextSong = songs[nextIndex].id
        let action = selectSongAction(nextSong)
        dispatch(action)

        navigationPath = [.song(nextSong)]
    }

    private func selectNextSong() {
        selectSongWithOffset(1)
    }

    private func offsetSongName(_ offset: Int) -> String? {
        let index = songs.firstIndex { $0.id == song.id }
        guard let index else { return nil }
        let nextIndex = index + offset
        guard 0 <= nextIndex && nextIndex < songs.count else { return nil }
        guard !songs[nextIndex].name.isEmpty else { return "Next" }
        return songs[nextIndex].name
    }

    private var nextSongName: String? {
        offsetSongName(1)
    }

    private var previousSongName: String? {
        offsetSongName(-1)
    }

    private func selectPreviousSong() {
        selectSongWithOffset(-1)
    }

    @ViewBuilder
    private var sidebar: some View {
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
    private var renameButton: some View {
        Button {
            editingName = true
        } label: {
            Label("Rename", systemImage: "pencil")
        }
    }

    @ViewBuilder
    private var sectionsButton: some View {
        Button {
            editingSections = true
        } label: {
            Label("Sections", systemImage: "rectangle.grid.1x2")
        }
    }

    @ViewBuilder
    private var addSampleButton: some View {
        Button {
            editingSample = true
        } label: {
            Label(
                song.sample == nil ? "Add Sample" : "Replace Sample",
                systemImage: "waveform"
            )
        }
    }

    @ViewBuilder
    private var tempoButton: some View {
        Button {
            editingTempo = true
        } label: {
            Label("Tempo", systemImage: "metronome")
        }
    }

    @ViewBuilder
    private var removeButton: some View {
        Button(role: .destructive) {
            let action = removeSongAction(song.id)
            dispatch(action)
        } label: {
            Label("Remove", systemImage: "trash")
        }
    }

    @ViewBuilder
    private var headerMenu: some View {
        Menu {
            renameButton
            sectionsButton
            addSampleButton
            tempoButton
            removeButton
        } label: {
            Image(systemName: "ellipsis")
        }
        .font(.title)
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

    @State static var navigationPath: [NavigationItem] = [.song(song.id)]

    static var previews: some View {
        SongView(
            song: song,
            songs: [song],
            selections: selections,
            playbackState: playbackState,
            progress: progress,
            waveforms: Waveforms(),
            dispatch: loggingDispatch,
            navigationPath: $navigationPath
        )
        .padding()

    }
}
