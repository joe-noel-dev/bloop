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

    case browse
    case setConnected(Bool)

    case uploadSample((Id, URL))
    case uploadAck(Id)

}
