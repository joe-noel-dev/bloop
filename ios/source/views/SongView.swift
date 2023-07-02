import SwiftUI

struct SongView: View {
    var song: Song

    var body: some View {
        VStack {
            ForEach(song.sections) { section in
                SectionView(section: section)
            }
        }.navigationTitle(song.name)
    }
}

struct SongView_Previews: PreviewProvider {
    static var previews: some View {
        SongView(song: demoSong(0))
    }
}
