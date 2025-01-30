import Foundation
import Network

class Discovery: NSObject {

    private var serviceBrowser: NWBrowser
    private var services: Set<NWEndpoint> = []
    var onKnownServersChanged: (([NWEndpoint]) -> Void)?
    var onScanning: ((Bool) -> Void)?
    private let resolveTimeout: TimeInterval = 10
    private var queue = DispatchQueue(label: "bloop.discovery")
    private var state: NWBrowser.State? = nil

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

        serviceBrowser.stateUpdateHandler = { [weak self] state in
            DispatchQueue.main.async {
                let wasScanning = self?.state == .ready
                self?.state = state
                let isScanning = state == .ready
                print("mDNS state: \(state)")

                if wasScanning != isScanning {
                    self?.onScanning?(isScanning)
                }

            }
        }

        self.serviceBrowser.start(queue: queue)
    }
}
