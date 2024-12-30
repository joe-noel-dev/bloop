import SwiftUI

struct SongsView: View {
    var project: Project
    var dispatch: Dispatch

    @Binding var navigationPath: [NavigationItem]

    var body: some View {
        NavigationView {
            List {
                ForEach(project.songs) { song in
                    SongRow(song: song) {
                        let action = selectSongAction(song.id)
                        dispatch(action)

                        navigationPath.append(.song(song.id))
                    }
                }
                .onMove { fromOffsets, toOffset in
                    var project = project
                    project.songs.move(fromOffsets: fromOffsets, toOffset: toOffset)

                    let action = updateProjectAction(project)
                    dispatch(action)
                }
                .onDelete { offsets in
                    var project = project
                    project.songs.remove(atOffsets: offsets)

                    let action = updateProjectAction(project)
                    dispatch(action)
                }

            }
            .toolbar {

                Button {
                    let action = addSongAction()
                    dispatch(action)
                } label: {
                    Label("Add Song", systemImage: "plus")
                }
            }
            .navigationTitle(project.info.name)
        }
    }
}

struct SongRow: View {
    var song: Song
    var action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(song.name)
                .frame(maxWidth: .infinity, alignment: .leading)
                .contentShape(Rectangle())
        }

    }
}

struct SongsView_Previews: PreviewProvider {
    static let project = demoProject()

    @State static var navigationPath: [NavigationItem] = []

    static var previews: some View {
        SongsView(project: project, dispatch: loggingDispatch, navigationPath: $navigationPath)
    }
}
