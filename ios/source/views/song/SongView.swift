import SwiftUI

struct SongView: View {
    var song: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    @State private var editingSong = false
    @State private var editingSections = false
    @State private var editingSample = false
    @State private var editingProjects = false
    @State private var editingProjectName = false
    @State private var newProjectName = ""

    init(song: Bloop_Song, state: AppState, dispatch: @escaping Dispatch) {
        self.song = song
        self.state = state
        self.dispatch = dispatch
        self.newProjectName = newProjectName
    }

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

    private var selectedSection: Bloop_Section? {
        song.sections.first {
            $0.id == state.project.selections.section
        }
    }

    private var isSelected: Bool {
        state.project.selections.song == song.id
    }

    private var isPlaying: Bool {
        state.playbackState.songID == song.id
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
        song.sample.id == 0 ? nil : song.sample.id
    }

    @ViewBuilder
    var sections: some View {
        LazyVGrid(columns: sectionColumns) {
            ForEach(song.sections) { section in
                SectionView(
                    section: section,
                    selections: state.project.selections,
                    playbackState: state.playbackState,
                    progress: state.progress,
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

                guard url.startAccessingSecurityScopedResource() else {
                    print("Failed to start security-scoped access.")
                    return
                }
                defer { url.stopAccessingSecurityScopedResource() }

                let tempDirectory = FileManager.default.temporaryDirectory
                let tempURL = tempDirectory.appendingPathComponent(url.lastPathComponent)

                do {
                    if FileManager.default.fileExists(atPath: tempURL.path) {
                        try FileManager.default.removeItem(at: tempURL)
                    }
                    try FileManager.default.copyItem(at: url, to: tempURL)
                    print("File copied to temporary location: \(tempURL)")
                }
                catch {
                    print("Failed to copy file to temporary location: \(error)")
                    return
                }

                let action = Action.uploadSample((song.id, tempURL))
                dispatch(action)

            case .failure(let error):
                print("Import failed: \(error)")
            }
        }

        .toolbar {
            MainToolbar(
                currentSong: song,
                editingSong: $editingSong,
                editingSections: $editingSections,
                editingSample: $editingSample,
                editingProjects: $editingProjects,
                editingProjectName: $editingProjectName,
                dispatch: dispatch
            )
        }
        .sheet(isPresented: $editingSong) {
            SongEditView(song) { newSong in
                if newSong != song {
                    let action = updateSongAction(newSong)
                    dispatch(action)
                }

                editingSong = false
            }
        }
        .sheet(isPresented: $editingProjects) {
            ProjectsView(projects: state.projects, dispatch: dispatch) {
                editingProjects = false
            }
        }
        .sheet(isPresented: $editingProjectName) {
            Form {
                Section("Project Name") {
                    TextEditor(text: $newProjectName)
                }

                Button("Save") {
                    let action = renameProjectAction(newProjectName)
                    dispatch(action)
                    editingProjectName = false
                }
            }
        }
        .navigationTitle(song.name)
    }

    private func selectSongWithOffset(_ offset: Int) {
        guard let index = state.project.songs.firstIndex(where: { $0.id == song.id }) else {
            return
        }

        let nextIndex = index + offset

        guard 0 <= nextIndex && nextIndex < state.project.songs.count else {
            return
        }

        let nextSong = state.project.songs[nextIndex].id
        let action = selectSongAction(nextSong)
        dispatch(action)
    }

    private func selectNextSong() {
        selectSongWithOffset(1)
    }

    private func offsetSongName(_ offset: Int) -> String? {
        let songs = state.project.songs
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
}

struct SongEditView: View {
    var song: Bloop_Song
    var onSubmit: (Bloop_Song) -> Void
    @State private var newSong: Bloop_Song

    init(_ song: Bloop_Song, onSubmit: @escaping (Bloop_Song) -> Void) {
        self.song = song
        self.onSubmit = onSubmit
        self.newSong = song

        if song.hasSample {
            newSong.tempo = song.sample.tempo
        }
    }

    var body: some View {
        Form {
            Section("Name") {
                TextField("Name", text: $newSong.name)
                    #if os(iOS)
                        .textInputAutocapitalization(.words)
                    #endif
                    .disableAutocorrection(true)
            }

            Section("Tempo") {
                TextField("Tempo", value: $newSong.tempo.bpm, formatter: NumberFormatter())
            }

            Button("Save") {
                submit()
            }
        }
        .onDisappear {
            submit()
        }
    }

    private func submit() {
        if newSong.hasSample {
            newSong.sample.tempo = newSong.tempo
        }

        onSubmit(newSong)
    }
}

struct SongView_Previews: PreviewProvider {

    static let state = {
        var appState = AppState()
        appState.project = demoProject()
        appState.progress = Bloop_Progress()
        return appState
    }()

    static let song: Bloop_Song = {
        state.project.songs[0]
    }()

    static var previews: some View {
        SongView(
            song: song,
            state: state,
            dispatch: loggingDispatch
        )
        .padding()

    }
}
