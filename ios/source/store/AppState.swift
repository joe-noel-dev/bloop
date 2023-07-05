import Foundation

struct AppState {
    var connected = false
    var projects: [ProjectInfo] = []
    var project = emptyProject()
    var playbackState = PlaybackState()
    var progress = Progress()
}

func emptyProject() -> Project {
    return Project(
        info: .init(id: "null", name: "", version: "", lastSaved: 0),
        songs: [],
        selections: Selections()
    )
}
