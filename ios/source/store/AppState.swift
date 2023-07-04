import Foundation

struct AppState {
    var connected = false
    var projects: [ProjectInfo] = []
    var project: Project?
    var playbackState = PlaybackState()
}
