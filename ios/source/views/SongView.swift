import SwiftUI

struct SongView: View {
    var song: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    @State private var editingEntity: EditingEntity?

    init(song: Bloop_Song, state: AppState, dispatch: @escaping Dispatch) {
        self.song = song
        self.state = state
        self.dispatch = dispatch
    }

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

    private var playingSection: Bloop_Section? {
        song.sections.first {
            $0.id == state.playbackState.sectionID
        }
    }

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



    var body: some View {
        ScrollViewReader { proxy in
            ScrollView(.vertical) {
                VStack(alignment: .leading) {
                    SectionsList(song: song, state: state, dispatch: dispatch)

                    Spacer()
                }
            }
            .padding(Layout.units(2))
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .contentShape(Rectangle())
            .onAppear {
                selectSong()
                scrollToPlayingSection(proxy: proxy)
            }
            .onChange(of: state.playbackState.sectionID) { _, _ in
                scrollToPlayingSection(proxy: proxy)
            }
            .onChange(of: state.playbackState.songID) { _, newSongID in
                if newSongID == song.id {
                    scrollToPlayingSection(proxy: proxy)
                }
            }
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
            MainToolbar(
                currentSong: song,
                servers: state.servers,
                scanning: state.scanning,
                editingEntity: $editingEntity
            ) { action in
                switch action {
                case .disconnect:
                    dispatch(.disconnect)
                case .connectToServer(let server):
                    dispatch(.disconnect)
                    dispatch(.connect(server))
                case .connectLocal:
                    dispatch(.disconnect)
                    dispatch(.connectLocal)
                }
            }
        }
        .sheet(isPresented: editingEntityBinding(.projects)) {
            ProjectsView(
                projects: state.projects,
                cloudProjects: state.cloudProjects,
                projectSyncStatuses: state.projectSyncStatuses,
                dispatch: dispatch
            ) {
                editingEntity = nil
            }
        }
        .sheet(isPresented: editingEntityBinding(.songs)) {
            SongsView(state: state, dispatch: dispatch)
        }
        .sheet(isPresented: editingEntityBinding(.settings)) {
            PreferencesView(preferences: state.preferences, audioDevices: state.audioDevices, audioStatus: state.audioStatus, dispatch: dispatch) {
                editingEntity = nil
            }
        }
        .navigationTitle(song.name)
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

    private func scrollToPlayingSection(proxy: ScrollViewProxy) {
        guard let playingSection = playingSection else { return }

        withAnimation(.easeInOut(duration: 0.5)) {
            proxy.scrollTo(playingSection.id, anchor: .center)
        }
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

struct SectionsList: View {
    var song: Bloop_Song
    var state: AppState
    var dispatch: Dispatch

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

    private var sectionColumns: [GridItem] {
        #if os(iOS)
            let columnCount = horizontalSizeClass == .compact ? 1 : 2
        #else
            let columnCount = 2
        #endif

        return Array(repeating: GridItem(.flexible()), count: columnCount)
    }

    var body: some View {
        LazyVGrid(columns: sectionColumns, spacing: Layout.units(2)) {
            ForEach(song.sections, id: \.id) { section in
                SectionView(
                    section: section,
                    selections: state.project.selections,
                    playbackState: state.playbackState,
                    progress: state.progress,
                    dispatch: dispatch
                )
                .id(section.id)
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
