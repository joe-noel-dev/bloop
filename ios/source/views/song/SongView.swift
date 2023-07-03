import SwiftUI

struct SongView: View {
    var song: Song
    var selections: Selections
    var dispatch: (Action) -> Void

    var selectedSection: Section? {
        song.sections.first {
            $0.id == selections.section
        }
    }

    private var isSelected: Bool {
        selections.song == song.id
    }

    func selectSong() {
        let request = Request.select(.init(entity: .song, id: song.id))
        dispatch(.sendRequest(request))
    }

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(song.name)
                    .font(.largeTitle)

                WaveformView()
                    .frame(height: 120)

                ForEach(song.sections) { section in
                    SectionView(section: section, dispatch: dispatch)
                }
            }

            Spacer()
        }
        .padding()
        .navigationTitle(song.name)
        .onTapGesture {
            if !isSelected {
                selectSong()
            }

        }
        .background(Colours.neutral1)
        .cornerRadius(Layout.cornerRadiusLarge)
    }
}

struct SongView_Previews: PreviewProvider {
    static var song: Song {
        var song = demoSong(0)
        song.name = "My Song Name"
        return song
    }

    static var selections: Selections {
        .init(song: song.id, section: song.sections[0].id)
    }

    static var previews: some View {
        SongView(song: song, selections: selections) { action in
            print("Dispatch: \(action)")
        }
        .padding()

    }
}
