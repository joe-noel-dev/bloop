import Foundation

enum Action {
    case sendRequest(Request)

    case setProject(Project)
    case setPlaybackState(PlaybackState)

    case connect(String)
    case setConnected(Bool)
}
