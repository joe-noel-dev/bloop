import Foundation

class ApiMiddleware: Middleware {
    private let core = Core()

    func execute(state: AppState, action: Action, dispatch: @escaping (Action) -> Void) {
        if case let sendRequestAction as SendRequestAction = action {
            core.sendRequest(sendRequestAction.request)
        }
    }

}
