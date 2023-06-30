import Foundation

func rootReducer(state: AppState, action: Action) -> AppState {
    var state = state

    if case let setProjectAction as SetProjectAction = action {
        state.project = setProjectAction.project
    }

    return state

}
