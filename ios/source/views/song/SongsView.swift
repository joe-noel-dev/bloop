import SwiftUI

struct SongsView: View {
    var state: AppState
    var dispatch: Dispatch

    private var selectedSongId: Binding<ID?> {
        Binding(
            get: { state.project.selections.song },
            set: { newValue in
                if let value = newValue {
                    let action = selectSongAction(value)
                    dispatch(action)
                }

            }
        )
    }

    private var songs: Binding<[Bloop_Song]> {
        Binding(
            get: { state.project.songs },
            set: { value in
                var project = state.project
                project.songs = value

                let action = updateProjectAction(project)
                dispatch(action)
            }
        )
    }

    var body: some View {
        NavigationView {
            List(songs, editActions: [.delete, .move]) { song in
                NavigationLink {
                    SongView(song: song.wrappedValue, state: state, dispatch: dispatch)
                } label: {
                    Text(song.wrappedValue.name)

                }
                .onTapGesture {
                    let action = selectSongAction(song.wrappedValue.id)
                    dispatch(action)
                }
            }
            .toolbar {
                EditButton()

                Button {
                    let action = addSongAction()
                    dispatch(action)
                } label: {
                    Label("Add Song", systemImage: "plus")
                }
            }
            .navigationTitle(state.project.info.name)
        }
    }
}

struct SongsView_Previews: PreviewProvider {
    static let project = demoProject()

    static let state: AppState = {
        var state = AppState()
        state.project = demoProject()
        return state
    }()

    static var previews: some View {
        SongsView(state: state, dispatch: loggingDispatch)
    }
}
