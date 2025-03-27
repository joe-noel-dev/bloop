import SwiftUI

struct SongView: View {
    var song: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    @Environment(\.editMode) var editMode

    @State var editSong: Bloop_Song

    @State private var editingSections = false
    @State private var editingSample = false
    @State private var editingProjects = false
    @State private var editingProjectName = false
    @State private var newProjectName = ""

    init(song: Bloop_Song, state: AppState, dispatch: @escaping Dispatch) {
        self.song = song
        self.state = state
        self.dispatch = dispatch
        self.editSong = song
        self.newProjectName = state.project.info.name
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

    private func onSampleSelected(_ result: Result<URL, any Error>) {
        switch result {
        case .success(let url):
            print("Selected URL for upload: \(url)")
            let action = Action.uploadSample((song.id, url))
            dispatch(action)
        case .failure(let error):
            print("Import failed: \(error)")
        }
    }

    private func onEditModeChanged(_ newValue: EditMode?) {
        if newValue == .active {
            editSong = song
        }
        else if newValue == .inactive {
            if editSong != song {
                let action = updateSongAction(editSong)
                dispatch(action)
            }
        }
    }

    var body: some View {

        VStack(alignment: .leading) {
            if editMode?.wrappedValue == .active {
                SongDetailsEditor(song: $editSong)
            }

            ScrollView(.vertical) {

                SectionsList(editSong: $editSong, state: state, dispatch: dispatch)
            }

            Spacer()
        }
        .padding(Layout.units(2))
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .contentShape(Rectangle())
        .onAppear {
            selectSong()
        }
        .onTapGesture {
            if !isSelected {
                selectSong()
            }
        }
        .gesture(
            songSwipeGesture
        )
        .sheet(isPresented: $editingSections) {
            SectionsView(song: song, dispatch: dispatch)
        }
        .fileImporter(isPresented: $editingSample, allowedContentTypes: [.wav]) { result in
            onSampleSelected(result)
        }
        .toolbar {
            ToolbarItemGroup(placement: .navigationBarLeading) {
                EditButton()

                Button(action: selectPreviousSong) {
                    Image(systemName: "chevron.backward")
                }
                .disabled(previousSongId(state.project) == nil)

                Button(action: selectNextSong) {
                    Image(systemName: "chevron.forward")
                }
                .disabled(nextSongId(state.project) == nil)
            }

            ToolbarItemGroup(placement: .navigationBarTrailing) {
                if editMode?.wrappedValue == .active {
                    Button(action: addNewSection) {
                        Label("Add Section", systemImage: "plus")
                    }
                }

                MainToolbar(
                    currentSong: song,
                    editingSections: $editingSections,
                    editingSample: $editingSample,
                    editingProjects: $editingProjects,
                    editingProjectName: $editingProjectName,
                    dispatch: dispatch
                )
            }
        }
        .sheet(isPresented: $editingProjects) {
            ProjectsView(projects: state.projects, dispatch: dispatch) {
                editingProjects = false
            }
        }
        .sheet(isPresented: $editingProjectName) {
            RenameProjectSheet(newProjectName: $newProjectName) {
                saveProjectName()
            }
        }
        .navigationTitle(song.name)
        .onChange(of: editMode?.wrappedValue) { oldValue, newValue in
            onEditModeChanged(newValue)
        }
        .onChange(of: song) { oldSong, newSong in
            if editMode?.wrappedValue != .active {
                editSong = newSong
            }
            else if editSong == oldSong {
                editSong = newSong
            }
        }
    }

    private var songSwipeGesture: some Gesture {
        DragGesture(minimumDistance: 20, coordinateSpace: .global).onEnded { value in
            if value.translation.width < 0 {
                selectNextSong()
            }

            if value.translation.width > 0 {
                selectPreviousSong()
            }
        }
    }

    private func addNewSection() {
        var section = Bloop_Section()
        section.id = randomId()
        section.name = "New Section"
        editSong.sections.append(section)
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

    private func saveProjectName() {
        let action = renameProjectAction(newProjectName)
        dispatch(action)
        editingProjectName = false
    }
}

private struct SongDetailsEditor: View {
    @Binding var song: Bloop_Song

    var body: some View {
        HStack {
            TextField("Name", text: $song.name)
                #if os(iOS)
                    .textInputAutocapitalization(.words)
                #endif
                .disableAutocorrection(true)

            TextField("Tempo", value: $song.tempo.bpm, formatter: NumberFormatter())
                .keyboardType(.decimalPad)
                .submitLabel(.done)
        }
    }
}

private struct RenameProjectSheet: View {
    @Binding var newProjectName: String
    var onSave: () -> Void

    var body: some View {
        Form {
            Section("Project Name") {
                TextEditor(text: $newProjectName)
            }

            Button("Save", action: onSave)
        }
    }
}

struct SectionsList: View {
    @Binding var editSong: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    var body: some View {
        LazyVStack(spacing: Layout.units(2)) {
            ForEach($editSong.sections) { section in
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
