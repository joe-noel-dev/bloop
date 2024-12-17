import Foundation
import Network

class Discovery: NSObject {

    private var serviceBrowser: NetServiceBrowser
    private var services: [NetService] = []
    var onServerDiscovered: ((Server) -> Void)?
    private let resolveTimeout: TimeInterval = 10
    private var discovered: [Server] = []

    override init() {
        serviceBrowser = NetServiceBrowser()
        super.init()
        serviceBrowser.includesPeerToPeer = true
        serviceBrowser.delegate = self
        
    }
    
    func browse() {
        services.removeAll()
        discovered.removeAll()
        serviceBrowser.searchForServices(ofType: "_bloop._tcp", inDomain: "local.")
    }

    func cancel() {
        serviceBrowser.stop()
    }

    private func notifyService(_ service: NetService) {
        guard let hostname = service.hostName, let onCoreDiscovered = self.onCoreDiscovered else {
            return
        }

        self.discovered.append((hostname, service.port))
        onCoreDiscovered(hostname, service.port)
    }
}

extension CoreDiscovery: NetServiceBrowserDelegate {
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
        
        self.discovered.removeAll { server in
            server.hostname == service.hostName && server.port == service.port
        }
    }
}

extension CoreDiscovery: NetServiceDelegate {
    func netServiceDidResolveAddress(_ sender: NetService) {
        notifyService(sender)
    }
}
