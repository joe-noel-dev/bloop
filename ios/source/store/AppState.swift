import Foundation

struct AppState {
    var connected: ConnectionType? = .none
    var scanning = false
    var servers: [Server] = []
    var projects: [ProjectInfo] = []
    var project = emptyProject()
    var playbackState = PlaybackState()
    var progress = Progress()
    var waveforms = Waveforms()
    var navigationPath: [NavigationItem] = []
}

func emptyProject() -> Project {
    return Project(
        info: .init(id: "null", name: "", version: "", lastSaved: 0),
        songs: [],
        selections: Selections()
    )
}
