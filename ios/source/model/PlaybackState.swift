import Foundation

struct PlaybackState: Codable {
    var playing = PlayingState.stopped
    var songId: Id?
    var sectionId: Id?
    var queuedSongId: Id?
    var queuedSectionId: Id?
    var looping: Bool = false
}

enum PlayingState: String, Codable {
    case stopped
    case playing
}
