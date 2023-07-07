import Foundation

enum Action {
    case sendRequest(Request)

    case setProject(Project)
    case setPlaybackState(PlaybackState)
    case setProgress(Progress)
    case setProjects([ProjectInfo])
    case addError(String)

    case connect(String)
    case setConnected(Bool)
}
