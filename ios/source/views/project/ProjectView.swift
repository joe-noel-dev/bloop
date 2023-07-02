import SwiftUI

struct ProjectView: View {
    var project: Project
    var dispatch: (Action) -> Void

    func addSong() {
        let request = Request.add(EntityId.init(entity: .song))
        dispatch(.sendRequest(request))
    }

    var body: some View {

        VStack(spacing: 16) {
            ForEach(project.songs) { song in
                SongView(song: song, selections: project.selections, dispatch: dispatch)
            }
            Spacer()
        }
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                Button(action: addSong) {
                    Image(systemName: "plus")
                }

            }
        }
        .navigationTitle(project.info.name)
        .padding()

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
