import Foundation

struct Project: Codable, Equatable {
    var info: ProjectInfo
    var songs: [Song]
    var selections: Selections
}

func selectedSongIndex(_ project: Project) -> Int? {
    let songId = project.selections.song
    let songs = project.songs
    return songs.firstIndex { song in
        song.id == songId
    }
}
