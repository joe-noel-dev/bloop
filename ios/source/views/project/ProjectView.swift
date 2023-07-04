import SwiftUI

struct ProjectView: View {
    var project: Project
    var playbackState: PlaybackState
    var progress: Progress
    var dispatch: Dispatch

    var body: some View {

        NavigationStack {
            ZStack {
                Colours.background.ignoresSafeArea()

                ScrollView(.vertical) {
                    VStack(spacing: Layout.units(4)) {
                        ForEach(project.songs) { song in
                            SongView(
                                song: song,
                                selections: project.selections,
                                playbackState: playbackState,
                                progress: progress,
                                dispatch: dispatch
                            )
                        }
                        Spacer()
                    }
                }
                .toolbar {
                    Button {
                        let action = addSongAction()
                        dispatch(action)
                    } label: {
                        Image(systemName: "plus")
                    }
                }
                .navigationTitle(project.info.name)
                .padding()
            }
        }
        .safeAreaInset(edge: .bottom) {

            TransportBar(
                playbackState: playbackState,
                selections: project.selections,
                dispatch: dispatch
            )

        }
    }
}

struct ProjectView_Previews: PreviewProvider {
    static let project = demoProject()
    static let playbackState = PlaybackState()
    static let progress = Progress()

    static var previews: some View {
        ProjectView(
            project: project,
            playbackState: playbackState,
            progress: progress,
            dispatch: loggingDispatch
        )
    }
}
