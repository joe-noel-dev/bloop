import SwiftUI

struct ProjectView: View {
    var state: AppState
    var dispatch: Dispatch

    @State private var projectsViewOpen = false
    @State private var editingProjectName = false
    @State private var editingSongs = false

    @State private var newProjectName = ""

    @Environment(\.colorScheme) var colorScheme

    var body: some View {
        NavigationStack {

            ScrollView(.vertical) {
                songViews
                    .padding()
            }
            .toolbar {
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
                        songsButton
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
            .background(colorScheme == .light ? Colours.backgroundLight : Colours.backgroundDark)
            .navigationTitle(state.project.info.name)
        }
        .safeAreaInset(edge: .bottom) {
            transportBar
        }
        .sheet(isPresented: $projectsViewOpen) {
            ProjectsView(projects: state.projects, dispatch: dispatch) {
                projectsViewOpen = false
            }
        }
        .sheet(isPresented: $editingSongs) {
            SongsView(project: state.project, dispatch: dispatch)
        }
    }

    @ViewBuilder
    private var songViews: some View {
        VStack(spacing: Layout.units(4)) {
            ForEach(state.project.songs) { song in
                SongView(
                    song: song,
                    selections: state.project.selections,
                    playbackState: state.playbackState,
                    progress: state.progress,
                    waveforms: state.waveforms,
                    dispatch: dispatch
                )
            }
            Spacer()
        }
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
            selections: state.project.selections,
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
}

struct ProjectView_Previews: PreviewProvider {
    static let state: AppState = .init(
        connected: true,
        projects: [],
        project: demoProject(),
        playbackState: PlaybackState(),
        progress: Progress()
    )

    static var previews: some View {
        ProjectView(
            state: state,
            dispatch: loggingDispatch
        )
    }
}
