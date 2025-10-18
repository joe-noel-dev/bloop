import Foundation

struct AppState {
    var connected: ConnectionType? = .none
    var scanning = false
    var servers: [Server] = []
    var projects: [Bloop_ProjectInfo] = []
    var cloudProjects: [Bloop_ProjectInfo] = []
    var project = emptyProject()
    var projectInfo: Bloop_ProjectInfo? = .none
    var projectSyncStatuses: [String: Bloop_SyncStatus] = [:]
    var playbackState = Bloop_PlaybackState()
    var progress = Bloop_Progress()
    var waveforms: [Id: Bloop_WaveformData] = [:]
    var user: Bloop_User? = .none
    var errors: [String] = []
    var preferences: Bloop_Preferences? = .none
}

func emptyProject() -> Bloop_Project {
    Bloop_Project.with {
        $0.songs = []
        $0.selections = Bloop_Selections()
    }
}
