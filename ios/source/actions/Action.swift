import Foundation

enum Action {
    case sendRequest(Request)

    case setProject(Project)
    case setPlaybackState(PlaybackState)
    case setProgress(Progress)
    case setProjects([ProjectInfo])
    case addWaveform((Id, WaveformData))
    case removeWaveform(Id)
    case addError(String)

    case setNavigationPath([NavigationItem])

    case connect(Server)
    case disconnect
    case setConnected(Bool)

    case uploadSample((Id, URL))
    case uploadAck(Id)

    case setDiscoveredServers([Server])
    case removeAllServers

    case restartScan

}
