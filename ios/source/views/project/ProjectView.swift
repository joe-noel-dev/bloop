import SwiftUI

struct ProjectView: View {
    var state: AppState
    var dispatch: Dispatch

    @State private var projectsViewOpen = false

    var body: some View {
        NavigationStack {

            ScrollView(.vertical) {
                songViews
                    .padding()
            }
            .toolbar {
                Menu {
                    addSongButton
                    openProjectButton
                    duplicateProjectButton
                } label: {
                    Image(systemName: "ellipsis")
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
    private var openProjectButton: some View {
        Button {
            projectsViewOpen = true
        } label: {
            Label("Projects", systemImage: "externaldrive")
        }
    }

    @ViewBuilder
    private var duplicateProjectButton: some View {
        Button {
            let action = duplicateProjectAction(state.project.info.id)
            dispatch(action)
        } label: {
            Label("Duplicate Project", systemImage: "doc.on.doc")
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
