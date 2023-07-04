import SwiftUI

struct ProjectView: View {
    var project: Project
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
                        SongView(song: song, selections: project.selections, dispatch: dispatch)
                    }
                    Spacer()
                }
            }
            .padding()

            TransportBar()
        }
        .background(Colours.neutral7)
    }
}

struct ProjectView_Previews: PreviewProvider {
    static let project = demoProject()

    static var previews: some View {
        ProjectView(project: project) { action in
            print("Dispatch: \(action)")
        }
    }
}
