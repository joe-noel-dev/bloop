import Foundation

func rootReducer(state: AppState, action: Action) -> AppState {
    var state = state

    switch action {
    case .setProject(let project):
        state.project = project

    case .setPlaybackState(let playbackState):
        state.playbackState = playbackState

    case .setConnected(let connected):
        state.connected = connected

    case .setProgress(let progress):
        state.progress = progress

    case .setProjects(let projects):
        state.projects = projects

    case .addError(let error):
        print("Error from core: \(error)")

    case .sendRequest, .connect:
        ()

    }

    return state
}
