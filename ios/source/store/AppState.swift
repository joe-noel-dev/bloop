import Foundation

struct AppState {
    var connected: ConnectionType? = .none
    var scanning = false
    var servers: [Server] = []
    var projects: [Bloop_ProjectInfo] = []
    var project = emptyProject()
    var playbackState = Bloop_PlaybackState()
    var progress = Bloop_Progress()
    var waveforms: [Id: Bloop_WaveformData] = [:]
    var errors: [String] = []
}

func emptyProject() -> Bloop_Project {
    Bloop_Project.with {

        $0.info = .with {
            $0.id = randomId()
            $0.name = ""
            $0.version = ""
            $0.lastSaved = 0
        }
        $0.songs = []
        $0.selections = Bloop_Selections()
    }
}
