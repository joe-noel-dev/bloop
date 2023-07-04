import Foundation

func rootReducer(state: AppState, action: Action) -> AppState {
    var state = state

    if case .setProject(let project) = action {
        state.project = project
    }

    if case .setPlaybackState(let playbackState) = action {
        state.playbackState = playbackState
    }

    if case .setConnected(let connected) = action {
        state.connected = connected
    }

    if case .setProgress(let progress) = action {
        state.progress = progress
    }

    return state
}
