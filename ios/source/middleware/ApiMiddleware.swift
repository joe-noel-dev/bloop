import Foundation

class ApiMiddleware: Middleware {
    private let core = Core()
    var dispatch: ((Action) -> Void)?

    init() {
        core.delegate = self
    }

    func execute(state: AppState, action: Action, dispatch: @escaping (Action) -> Void) {
        self.dispatch = dispatch

        if case .connect(let ipAddress) = action {
            core.connect(ipAddress)
        }

        if case .sendRequest(let request) = action {
            core.sendRequest(request)
        }
    }

}

extension ApiMiddleware: CoreDelegate {
    func coreConnected() {
        self.dispatch?(.setConnected(true))
    }

    func coreDisconnected() {
        self.dispatch?(.setConnected(false))
    }

    func coreDidSendResponse(_ response: Response) {
        if let project = response.project {
            self.dispatch?(.setProject(project))
        }
    }

}
