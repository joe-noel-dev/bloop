import SwiftUI

struct SongsView: View {
    var state: AppState
    var dispatch: Dispatch

    private var songs: [Bloop_Song] {
        state.project.songs
    }

    var body: some View {
        NavigationView {
            List {
                ForEach(songs) { song in
                    HStack {
                        Text(song.name)
                            .font(.body)
                            .foregroundColor(
                                song.id == state.project.selections.song ? .accentColor : .primary
                            )
                            .padding(.vertical, 6)

                        Spacer()

                        if song.id == state.project.selections.song {
                            Image(systemName: "checkmark.circle.fill")
                                .foregroundColor(.accentColor)
                        }
                        else {
                            Image(systemName: "chevron.right")
                                .foregroundColor(.secondary)
                                .opacity(0.4)
                        }
                    }
                    .contentShape(Rectangle())
                    .onTapGesture {
                        dispatch(selectSongAction(song.id))
                    }
                }
            }
            .listStyle(.insetGrouped)
            .navigationTitle("Songs")
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
