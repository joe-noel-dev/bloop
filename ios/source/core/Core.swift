import Foundation

protocol CoreDelegate: AnyObject {
    func coreConnected()
    func coreDisconnected()
    func coreDidSendResponse(_ data: Data)

    func onKnownServersChanged(_ servers: [Server])
    func onScanning(_ scanning: Bool)
}

class Core: CoreConnectionDelegate {
    private let connection = CoreConnection()
    weak var delegate: CoreDelegate?

    private let group = DispatchGroup()
    private let queue = DispatchQueue(label: "CoreEncodeDecode", attributes: .concurrent)
    private var discovery: Discovery? = nil

    init() {
        connection.delegate = self

        restartScan()
    }

    func connect(_ server: Server) {
//        if self.connection.state == .disconnected {
            self.connection.connect(server)
//        }
    }

    func disconnect() {
        self.connection.disconnect()
    }

    func sendRequest(_ request: Data) {
        queue.async {
            self.send(request)
        }
    }

    func restartScan() {
        discovery = Discovery()

        discovery?.onKnownServersChanged = { servers in
            DispatchQueue.main.async { [weak self] in
                self?.delegate?.onKnownServersChanged(servers)
            }
        }

        discovery?.onScanning = { scanning in
            DispatchQueue.main.async {
                [weak self] in
                self?.delegate?.onScanning(scanning)
            }
        }
    }

    private func send(_ data: Data) {
        DispatchQueue.main.async {
            self.connection.send(data)
        }
    }

    private func onResponse(_ data: Data) {
        DispatchQueue.main.async { [weak self] in
            self?.delegate?.coreDidSendResponse(data)
        }
    }
}

extension Core {
    func coreConnectionDidConnect() {
        print("Core connected")
        self.delegate?.coreConnected()
    }

    func coreConnectionDidDisconnect() {
        print("Core disconnected")
        self.delegate?.coreDisconnected()
    }

    func coreConnectionDidReceiveData(data: Data) {
        queue.async { [weak self] in
            self?.onResponse(data)
        }

    }

    func coreConnectionDidReceiveString(string: String) {
        print("Received: \(string)")
    }
}
