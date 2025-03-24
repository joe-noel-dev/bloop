import SwiftUI

struct ProjectView: View {
    var state: AppState
    var dispatch: Dispatch

    @State private var projectsViewOpen = false
    @State private var editingProjectName = false
    @State private var editingSongs = false

    @State private var newProjectName = ""
    
    private var selectedSong: Bloop_Song? {
        let selectedSongId = state.project.selections.song
        return state.project.songs.first {
            $0.id == selectedSongId
        }
    }

    @Environment(\.colorScheme) var colorScheme
    var body: some View {

        VStack(spacing: 0) {
            NavigationSplitView {
                SongsView(
                    state: state,
                    dispatch: dispatch
                )
            } detail: {
                if let selectedSong = self.selectedSong {
                    SongView(song: selectedSong, state: state, dispatch: dispatch)
                }
            }
            .toolbar {
                toolbarContent
            }
            .sheet(isPresented: $projectsViewOpen) {
                ProjectsView(projects: state.projects, dispatch: dispatch) {
                    projectsViewOpen = false
                }
            }
            .frame(maxHeight: .infinity)
            
            transportBar
        }
    }

    @ToolbarContentBuilder private var toolbarContent: some ToolbarContent {
        #if os(iOS)
            ToolbarItem(placement: .navigationBarLeading) {
                MetronomeView(
                    isPlaying: state.playbackState.playing == .playing,
                    sectionBeat: Int(floor(state.progress.sectionBeat))
                )
            }
        #endif

        ToolbarItem {
            Menu {
                projectsButton
                renameProjectButton
                disconnectButton
            } label: {
                Image(systemName: "ellipsis")
            }
            .popover(isPresented: $editingProjectName) {
                NameEditor(prompt: "Project Name", value: $newProjectName)
                    .onSubmit {
                        let action = renameProjectAction(newProjectName)
                        dispatch(action)
                        editingProjectName = false
                    }
                    .onAppear {
                        newProjectName = state.project.info.name
                    }
            }
        }
    }

    @ViewBuilder private var scrollView: some View {
        ScrollView(.horizontal) {
            LazyHStack {
                songViews
            }
        }
    }

    @ViewBuilder
    private var songViews: some View {

        TabView {
            ForEach(state.project.songs) { song in
                SongView(
                    song: song,
                    state: state,
                    dispatch: dispatch
                )
            }
        }
        .frame(width: UIScreen.main.bounds.width)
        .tabViewStyle(.page)

    }

    @ViewBuilder
    private var songsButton: some View {
        Button {
            editingSongs = true
        } label: {
            Label("Songs", systemImage: "music.note.list")
        }
    }

    @ViewBuilder
private var transportBar: some View {
        TransportBar(
            playbackState: state.playbackState,
            project: state.project,
            dispatch: dispatch
        )
    }

    @ViewBuilder
    private var projectsButton: some View {
        Button {
            projectsViewOpen = true
        } label: {
            Label("Projects", systemImage: "externaldrive")
        }
    }

    @ViewBuilder
    private var renameProjectButton: some View {
        Button {
            editingProjectName = true
        } label: {
            Label("Rename Project", systemImage: "pencil")
        }
    }

    @ViewBuilder
    private var disconnectButton: some View {
        Button {
            dispatch(.disconnect)
        } label: {
            Label("Disconnect", systemImage: "phone.down.fill")
        }
    }
}

struct ProjectView_Previews: PreviewProvider {
    static let state: AppState = .init(
        connected: .remote,
        projects: [],
        project: demoProject(),
        playbackState: Bloop_PlaybackState(),
        progress: Bloop_Progress()
    )

    static var previews: some View {
        ProjectView(
            state: state,
            dispatch: loggingDispatch
        )
    }
}
