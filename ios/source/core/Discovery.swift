import Foundation
import Network

class Discovery: NSObject {

    private var serviceBrowser: NetServiceBrowser
    private var services: [NetService] = []
    var onServerDiscovered: ((Server) -> Void)?
    private let resolveTimeout: TimeInterval = 10

    override init() {
        serviceBrowser = NetServiceBrowser()
        super.init()
        serviceBrowser.includesPeerToPeer = true
        serviceBrowser.delegate = self

    }

    func browse() {
        serviceBrowser.searchForServices(ofType: "_bloop._tcp", inDomain: "local.")
    }

    func cancel() {
        serviceBrowser.stop()
    }

    private func notifyService(_ service: NetService) {
        guard let hostname = service.hostName, let onServerDiscovered = self.onServerDiscovered
        else {
            return
        }

        let server = Server.init(hostname: hostname, port: service.port)
        onServerDiscovered(server)
    }
}

extension Discovery: NetServiceBrowserDelegate {
    func netServiceBrowser(
        _ browser: NetServiceBrowser,
        didFind service: NetService,
        moreComing: Bool
    ) {
        services.append(service)
        service.delegate = self
        service.resolve(withTimeout: resolveTimeout)
    }

    func netServiceBrowser(
        _ browser: NetServiceBrowser,
        didRemove service: NetService,
        moreComing: Bool
    ) {
        services.removeAll(where: { service == $0 })
    }
}

extension Discovery: NetServiceDelegate {
    func netServiceDidResolveAddress(_ sender: NetService) {
        notifyService(sender)
    }
}
