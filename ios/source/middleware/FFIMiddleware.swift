import Foundation

class FFIMiddleware: Middleware {
    var dispatch: Dispatch?
    private var coreFFI: CoreFFI?

    func execute(state: AppState, action: Action) {

        if case .connectLocal = action {
            initialiseCore()

            self.dispatch?(.setConnected(.local))
            self.dispatch?(.sendRequest(.with {
                $0.get = .with {
                    $0.entity = .all
                }
            }))
        }

        if case .disconnect = action {
            if state.connected == .local {
                self.dispatch?(.setConnected(.none))
            }
        }

        if case .sendRawRequest(let request) = action {
            if state.connected == .local {
                sendRequest(request)
            }
        }
    }

    private func initialiseCore() {
        if coreFFI != nil {
            return
        }

        coreFFI = CoreFFI(responseHandler: { [weak self] response in
            DispatchQueue.main.async {
                self?.dispatch?(.receivedRawResponse(response))
            }
        })
    }

    private func shutDownCore() {
        coreFFI = nil
    }

    private func sendRequest(_ request: Data) {
        do {
            try coreFFI?.addRequest(request)
        }
        catch {
            print("Error adding request via FFI: \(error)")
        }

    }

}
