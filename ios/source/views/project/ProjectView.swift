import SwiftUI

struct ProjectView: View {
    var state: AppState
    var dispatch: Dispatch

    @State private var projectsViewOpen = false
    @State private var editingProjectName = false
    @State private var newProjectName = ""

    var body: some View {
        NavigationStack {

            ScrollView(.vertical) {
                songViews
                    .padding()
            }
            .toolbar {
                Menu {
                    projectsButton
                    addSongButton
                    renameProjectButton
                } label: {
                    Image(systemName: "ellipsis")
                }
                .popover(isPresented: $editingProjectName) {
                    NameEditor(value: $newProjectName)
                        .onSubmit {
                            let action = renameProjectAction(newProjectName)
                            dispatch(action)
                            editingProjectName = false
                        }
                }

            }
            .navigationTitle(state.project.info.name)
            .background(.thickMaterial)

        }
        .safeAreaInset(edge: .bottom) {
            transportBar
        }

        .sheet(isPresented: $projectsViewOpen) {
            ProjectsView(projects: state.projects, dispatch: dispatch) {
                projectsViewOpen = false
            }
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
    private var addSongButton: some View {
        Button {
            let action = addSongAction()
            dispatch(action)
        } label: {
            Label("Add Song", systemImage: "plus")
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
