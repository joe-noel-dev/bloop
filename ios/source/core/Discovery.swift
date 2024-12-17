import Foundation
import Network

class Discovery: NSObject {

    private var serviceBrowser: NWBrowser
    private var services: Set<NWEndpoint> = []
    var onKnownServersChanged: (([NWEndpoint]) -> Void)?
    private let resolveTimeout: TimeInterval = 10
    private var browsing = false

    override init() {
        let parameters = NWParameters()
        parameters.includePeerToPeer = true

        let descriptor = NWBrowser.Descriptor.bonjour(type: "_bloop._tcp", domain: "local.")

        serviceBrowser = NWBrowser(for: descriptor, using: parameters)

        super.init()

        serviceBrowser.browseResultsChangedHandler = { [weak self] results, changes in
            DispatchQueue.main.async {
                self?.services = Set(results.map { $0.endpoint })
                self?.onKnownServersChanged?(results.map { $0.endpoint })
            }
        }
    }

    func browse() {
        if browsing {
            return
        }

        serviceBrowser.start(queue: .main)
        browsing = true
    }

    func cancel() {
        if !browsing {
            return
        }

        serviceBrowser.cancel()
        browsing = false
    }
}
