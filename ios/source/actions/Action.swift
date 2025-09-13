import Foundation

enum ConnectionType {
    case local
    case remote
}

enum Action {
    case sendRequest(Bloop_Request)
    case receivedResponse(Bloop_Response)

    case sendRawRequest(Data)
    case receivedRawResponse(Data)

    case setProject(Bloop_Project)
    case setPlaybackState(Bloop_PlaybackState)
    case setProgress(Bloop_Progress)
    case setProjects([Bloop_ProjectInfo])
    case setCloudProjects([Bloop_ProjectInfo])
    case setProjectInfo(Bloop_ProjectInfo)
    case setProjectSync(String, Bloop_SyncStatus)
    case dismissProjectSync(String)
    case addWaveform((Id, Bloop_WaveformData))
    case removeWaveform(Id)
    case setUser(Bloop_User)
    case clearUser
    case addError(String)

    case connect(Server)
    case connectLocal
    case disconnect
    case setConnected(ConnectionType?)

    case uploadSample((Id, URL))
    case uploadAck(Id)

    case setDiscoveredServers([Server])
    case setScanning(Bool)
    case removeAllServers

    case restartScan
}
