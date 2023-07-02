import SwiftUI

struct SongOverview: View {
    var song: Song
    var selections: Selections
    var dispatch: (Action) -> Void

    var isSelected: Bool {
        selections.song == song.id
    }

    func removeSong() {
        let request = Request.remove(EntityId.init(entity: .song, id: song.id))
        dispatch(.sendRequest(request))
    }

    var body: some View {
        HStack {
            Text(song.name)
                .font(.title)
            Spacer()
            NavigationLink(value: song) {
                Image(systemName: "pencil")
            }
        }
        .padding()
        .background(isSelected ? Colours.theme1 : Colours.neutral1)
        .onTapGesture {
            let request = Request.select(EntityId.init(entity: .song, id: song.id))
            dispatch(.sendRequest(request))
        }
        .cornerRadius(Layout.cornerRadiusLarge)
        .contextMenu {
            Button {
                removeSong()
            } label: {
                Label("Remove Song", systemImage: "trash")
            }
        }

    }
}

struct SongOverview_Previews: PreviewProvider {
    static var song = {
        var song = demoSong(0)
        song.name = "Song name"
        return song
    }()

    static var previews: some View {
        SongOverview(song: song, selections: Selections.init(song: song.id)) { action in
            print("Dispatch: \(action)")
        }
        .previewLayout(.sizeThatFits)
        .padding()
    }
}
