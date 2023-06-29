import Foundation

struct PlaybackState: Codable {
    var playing: PlayingState
    var songId: Id?
    var sectionId: Id?
    var queuedSongId: Id?
    var queuedSectionId: Id?
    var looping: Bool
}

enum PlayingState: String, Codable {
    case stopped
    case playing
}
