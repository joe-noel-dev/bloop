import Foundation

func rootReducer(state: AppState, action: Action) -> AppState {
    var state = state

    if case .setProject(let project) = action {
        state.project = project
    }

    if case .setConnected(let connected) = action {
        state.connected = connected
    }

    return state
}
