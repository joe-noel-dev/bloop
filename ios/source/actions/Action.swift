import Foundation

enum Action {
    case sendRequest(Request)
    case receivedResponse(Response)
    
    case sendRawRequest(Data)
    case receivedRawResponse(Data)

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
    case setScanning(Bool)
    case removeAllServers

    case restartScan

}
