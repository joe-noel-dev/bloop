import SwiftUI

struct ProjectView: View {
    var project: Project
    var playbackState: PlaybackState
    var dispatch: Dispatch

    func addSong() {
        let request = Request.add(EntityId.init(entity: .song))
        dispatch(.sendRequest(request))
    }

    var body: some View {

        VStack {
            ScrollView(.vertical) {
                VStack(spacing: Layout.units(4)) {
                    ForEach(project.songs) { song in
                        SongView(
                            song: song,
                            selections: project.selections,
                            playbackState: playbackState,
                            dispatch: dispatch
                        )
                    }
                    Spacer()
                }
            }
            .padding()
        }
        .background(Colours.background)
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

    static var previews: some View {
        ProjectView(project: project, playbackState: playbackState, dispatch: loggingDispatch)
    }
}
