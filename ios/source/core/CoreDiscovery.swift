import Foundation
import Network

class CoreDiscovery: NSObject, NetServiceBrowserDelegate, NetServiceDelegate {

    private var serviceBrowser: NetServiceBrowser?
    private var services: [NetService] = []
    var onCoreDiscovered: ((_ ipAddress: String, _ port: Int) -> Void)?
    private let resolveTimeout: TimeInterval = 10

    func browse() {
        let serviceBrowser = NetServiceBrowser()
        serviceBrowser.includesPeerToPeer = true
        serviceBrowser.delegate = self
        serviceBrowser.searchForServices(ofType: "_bloop._tcp", inDomain: "local.")

        self.serviceBrowser = serviceBrowser
    }

    func cancel() {
        if let serviceBrowser = self.serviceBrowser {
            serviceBrowser.stop()
        }

        self.serviceBrowser = nil
    }

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

    func netServiceDidResolveAddress(_ sender: NetService) {
        notifyService(sender)
    }

    private func notifyService(_ service: NetService) {
        guard let hostname = service.hostName, let onCoreDiscovered = self.onCoreDiscovered else {
            return
        }

        onCoreDiscovered(hostname, service.port)
    }
}
