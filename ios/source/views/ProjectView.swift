import SwiftUI

struct ProjectView: View {
    var project: Project
    var dispatch: (Action) -> Void

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                ForEach(project.songs) { song in
                    SongOverview(song: song, selections: project.selections, dispatch: dispatch)

                }
                Spacer()
            }
            .navigationDestination(for: Song.self) { song in
                SongView(song: song)
            }
            .navigationTitle(project.info.name)
            .padding()
        }

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
