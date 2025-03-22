import Foundation

class ApiMiddleware: Middleware {
    var dispatch: Dispatch?
    private let core = Core()

    init() {
        core.delegate = self
    }

    func execute(state: AppState, action: Action) {
        if case .connect(let server) = action {
            core.connect(server)
        }

        if case .disconnect = action {
            core.disconnect()
        }

        if case .sendRawRequest(let data) = action {
            core.sendRequest(data)
        }

        if case .restartScan = action {
            core.restartScan()
        }
    }

}

extension ApiMiddleware: CoreDelegate {
    func coreConnected() {
        self.dispatch?(.setConnected(.remote))

        self.dispatch?(
            .sendRequest(
                .with {
                    $0.get = .with {
                        $0.entity = .all
                    }
                }
            )
        )
    }

    func coreDisconnected() {
        self.dispatch?(.setConnected(.none))
    }

    func coreDidSendResponse(_ response: Data) {
        self.dispatch?(.receivedRawResponse(response))
    }

    func onKnownServersChanged(_ servers: [Server]) {
        self.dispatch?(.setDiscoveredServers(servers))
    }

    func onScanning(_ scanning: Bool) {
        self.dispatch?(.setScanning(scanning))
    }
}
