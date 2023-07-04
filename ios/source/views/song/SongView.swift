import SwiftUI

struct SongView: View {
    var song: Song
    var selections: Selections
    var dispatch: Dispatch
    @Environment(\.horizontalSizeClass) var horizontalSizeClass

    private var selectedSection: Section? {
        song.sections.first {
            $0.id == selections.section
        }
    }

    private var isSelected: Bool {
        selections.song == song.id
    }

    private func selectSong() {
        let action = selectSongAction(song.id)
        dispatch(action)
    }

    private var sectionColumns: [GridItem] {
        let columnCount = horizontalSizeClass == .compact ? 1 : 2

        return Array(repeating: GridItem(.flexible()), count: columnCount)
    }

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(song.name)
                    .font(.largeTitle)

                WaveformView()
                    .frame(height: 120)

                LazyVGrid(columns: sectionColumns) {
                    ForEach(song.sections) { section in
                        SectionView(
                            section: section,
                            isSelected: selections.isSectionSelected(sectionId: section.id),
                            dispatch: dispatch
                        )
                    }
                }

            }

            Spacer()
        }
        .padding()
        .overlay(alignment: .leading) {
            if isSelected {
                Colours.theme2
                    .frame(width: Layout.units(0.5))
            }
        }
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
    static let song: Song = {
        var song = demoSong(0)
        song.name = "My Song Name"
        return song
    }()

    static let selections: Selections = {
        .init(song: song.id, section: song.sections[0].id)
    }()

    static var previews: some View {
        SongView(song: song, selections: selections) { action in
            print("Dispatch: \(action)")
        }
        .padding()

    }
}
