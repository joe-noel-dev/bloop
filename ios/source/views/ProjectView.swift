import SwiftUI

struct ProjectView: View {
    var state: AppState
    var dispatch: Dispatch

    @State private var newProjectName = ""

    init(state: AppState, dispatch: @escaping Dispatch) {
        self.state = state
        self.dispatch = dispatch
        self.newProjectName = state.project.info.name
    }

    private var selectedSong: Bloop_Song? {
        let selectedSongId = state.project.selections.song
        return state.project.songs.first {
            $0.id == selectedSongId
        }
    }

    @Environment(\.colorScheme) var colorScheme

    @ViewBuilder
    var body: some View {

        NavigationStack {
            VStack(spacing: 0) {

                if let selectedSong = self.selectedSong {
                    SongView(song: selectedSong, state: state, dispatch: dispatch)
                        .frame(maxHeight: .infinity)
                }
                else {
                    Spacer()
                }

                TransportBar(
                    playbackState: state.playbackState,
                    project: state.project,
                    dispatch: dispatch
                )
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
