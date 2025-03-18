import Foundation

class FFIMiddleware: Middleware {
    var dispatch: Dispatch?

    private lazy var coreFFI: CoreFFI = {
        // FIXME: Hack to mark as connected
        self.dispatch?(.setConnected(true))

        return CoreFFI(responseHandler: { [weak self] response in
            DispatchQueue.main.async {
                self?.dispatch?(.receivedRawResponse(response))
            }
        })!
    }()

    func execute(state: AppState, action: Action) {
        // FIXME: Hack to initialize
        if case .restartScan = action {
            self.dispatch?(.sendRequest(.get(EntityId(entity: .all))))
        }

        if case .sendRawRequest(let request) = action {
            sendRequest(request)
        }
    }

    private func sendRequest(_ request: Data) {
        do {
            try coreFFI.addRequest(request)
        }
        catch {
            print("Error adding request via FFI: \(error)")
        }

    }

}
