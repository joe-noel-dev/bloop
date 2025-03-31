import SwiftUI

struct SongView: View {
    var song: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    @Environment(\.editMode) var editMode

    @State var editSong: Bloop_Song

    @State private var editingEntity: EditingEntity?
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

        ScrollView(.vertical) {
            VStack(alignment: .leading) {
                if editMode?.wrappedValue == .active {
                    SongDetailsEditor(song: $editSong)

                    SampleDetailsEditor(song: editSong, dispatch: dispatch)
                }

                SectionsList(editSong: $editSong, state: state, dispatch: dispatch)

                Spacer()
            }
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

            MainToolbar(
                currentSong: song,
                editingEntity: $editingEntity
            ) { action in
                switch action {
                case .disconnect:
                    dispatch(.disconnect)
                }
            }
        }
        .sheet(isPresented: editingEntityBinding(.projects)) {
            ProjectsView(projects: state.projects, dispatch: dispatch) {
                editingEntity = nil
            }
        }
        .sheet(isPresented: $editingProjectName) {
            RenameProjectSheet(newProjectName: $newProjectName) {
                saveProjectName()
            }.onAppear {
                newProjectName = state.project.info.name
            }
        }
        .sheet(isPresented: editingEntityBinding(.songs)) {
            SongsView(state: state, dispatch: dispatch)
        }
        .navigationTitle(editMode?.wrappedValue == .active ? editSong.name : song.name)
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

    private func editingEntityBinding(_ entity: EditingEntity) -> Binding<Bool> {
        Binding(
            get: {
                editingEntity == entity
            },
            set: { value in
                if value {
                    editingEntity = entity
                }
                else {
                    editingEntity = nil
                }

            }
        )
    }
}

private struct SampleDetailsEditor: View {
    var song: Bloop_Song
    var dispatch: Dispatch

    @State var editingSample: Bool = false

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: Layout.units(1)) {
                Text("Sample Details")
                    .font(.title2)
                    .padding(.bottom, Layout.units(0.5))

                if song.hasSample {

                    Text("Name")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Text(song.sample.name)

                }

                HStack {
                    Button(
                        song.hasSample ? "Replace Sample" : "Add Sample",
                        systemImage: song.hasSample ? "arrow.2.squarepath" : "plus"
                    ) {
                        editingSample = true
                    }
                    .buttonStyle(.bordered)

                    if song.hasSample {
                        Button(role: .destructive) {
                            var newSong = song
                            newSong.clearSample()
                            dispatch(updateSongAction(newSong))
                        } label: {
                            Label("Remove Sample", systemImage: "trash")
                        }
                        .buttonStyle(.bordered)
                    }
                }
            }

            Spacer()
        }
        .frame(maxWidth: .infinity)
        .padding(Layout.units(2))
        .background(Color(.secondarySystemBackground))
        .cornerRadius(Layout.corderRadiusMedium)
        .fileImporter(isPresented: $editingSample, allowedContentTypes: [.wav]) { result in
            onSampleSelected(result)
        }
    }

    private func onSampleSelected(_ result: Result<URL, any Error>) {
        switch result {
        case .success(let url):
            print("Selected URL for upload: \(url)")
            dispatch(.uploadSample((song.id, url)))
        case .failure(let error):
            print("Import failed: \(error)")
        }
    }
}

private struct SongDetailsEditor: View {
    @Binding var song: Bloop_Song
    @FocusState private var focusedField: Field?

    private enum Field: Hashable {
        case name
        case tempo
    }

    var body: some View {
        VStack(alignment: .leading, spacing: Layout.units(2)) {
            Text("Song Details")
                .font(.title2)
                .padding(.bottom, Layout.units(0.5))

            HStack(spacing: Layout.units(1.5)) {
                VStack(alignment: .leading, spacing: Layout.units(0.5)) {
                    Text("Name")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    TextField("Enter song name", text: $song.name)
                        .textFieldStyle(.roundedBorder)
                        #if os(iOS)
                            .textInputAutocapitalization(.words)
                        #endif
                        .disableAutocorrection(true)
                        .focused($focusedField, equals: .name)
                        .submitLabel(.next)
                }

                VStack(alignment: .leading, spacing: Layout.units(0.5)) {
                    Text("Tempo (BPM)")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    TextField("120", value: $song.tempo.bpm, formatter: NumberFormatter())
                        .keyboardType(.decimalPad)
                        .textFieldStyle(.roundedBorder)
                        .focused($focusedField, equals: .tempo)
                        .submitLabel(.done)
                }
            }
        }
        .padding(Layout.units(2))
        .background(Color(.secondarySystemBackground))
        .cornerRadius(Layout.corderRadiusMedium)
        .onSubmit {
            switch focusedField {
            case .name:
                focusedField = .tempo
            default:
                focusedField = nil
            }
        }
    }
}

private struct RenameProjectSheet: View {
    @Binding var newProjectName: String
    var onSave: () -> Void

    var body: some View {
        Form {
            Section("Project Name") {
                TextField("Project Name", text: $newProjectName)
            }

            Button("Save", action: onSave)
        }
    }
}

struct SectionsList: View {
    @Binding var editSong: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    @Environment(\.editMode) private var editMode

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

            if editMode?.wrappedValue == .active {
                Button("Add Section") {
                    let section = Bloop_Section.with {
                        $0.id = randomId()
                        $0.name = "Section"
                        $0.start = (editSong.sections.last?.start ?? 0.0) + 16.0
                    }

                    editSong.sections.append(section)
                }
            }
        }
    }

}

private let state = {
    var appState = AppState()
    appState.project = demoProject()
    appState.progress = Bloop_Progress()
    return appState
}()

private let song: Bloop_Song = {
    state.project.songs[0]
}()

#Preview {
    SongView(
        song: song,
        state: state,
        dispatch: loggingDispatch
    )
    .padding()
}

#Preview {
    SongView(song: song, state: state, dispatch: loggingDispatch).padding().environment(
        \.editMode,
        .constant(.active)
    )
}

#Preview {
    SongView(song: song, state: state, dispatch: loggingDispatch).padding().environment(
        \.colorScheme,
        .dark
    ).environment(\.editMode, .constant(.active))
}
