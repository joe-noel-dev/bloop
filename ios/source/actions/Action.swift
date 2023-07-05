import Foundation

enum Action {
    case sendRequest(Request)

    case setProject(Project)
    case setPlaybackState(PlaybackState)
    case setProgress(Progress)
    case setProjects([ProjectInfo])

    case connect(String)
    case setConnected(Bool)
}
