import Foundation
import SwiftBSON

protocol CoreDelegate: AnyObject {
    func coreConnected()
    func coreDisconnected()
    func coreDidSendResponse(_ response: Response)
}

class Core: CoreConnectionDelegate {
    private let connection = CoreConnection()
    weak var delegate: CoreDelegate?

    private let group = DispatchGroup()
    private let queue = DispatchQueue(label: "CoreEncodeDecode", attributes: .concurrent)
    private let discovery = CoreDiscovery()

    init() {
        connection.delegate = self

        discovery.onCoreDiscovered = { hostname, port in
            self.connect(hostname: hostname, port: port)
        }

        discovery.browse()
    }

    func browse() {
        discovery.browse()
    }

    private func connect(hostname: String, port: Int) {
        if self.connection.state == .disconnected {
            self.connection.connect(hostname: hostname, port: port)
        }
    }

    func sendRequest(_ request: Request) {
        queue.async {
            do {
                let encodedRequest = try BSONEncoder().encode(request)
                let data = encodedRequest.toData()
                self.send(data)
            }
            catch {
                print("Error sending request: \(error)")
            }
        }

    }

    private func send(_ data: Data) {
        DispatchQueue.main.async {
            self.connection.send(data)
        }
    }

    private func onResponse(_ response: Response) {
        DispatchQueue.main.async { [weak self] in
            self?.delegate?.coreDidSendResponse(response)
        }
    }
}

extension Core {
    func coreConnectionDidConnect() {
        print("Core connected")

        let getAllRequest = Request.get(EntityId(entity: .all))
        sendRequest(getAllRequest)

        self.delegate?.coreConnected()
    }

    func coreConnectionDidDisconnect() {
        print("Core disconnected")

        self.delegate?.coreDisconnected()
        self.discovery.browse()
    }

    func coreConnectionDidReceiveData(data: Data) {
        queue.async { [weak self] in
            do {
                let bsonDocument = try BSONDocument(fromBSON: data)
                let response = try BSONDecoder().decode(Response.self, from: bsonDocument)
                self?.onResponse(response)
            }
            catch {
                print("Error decoding response from core: \(error)")
            }
        }

    }

    func coreConnectionDidReceiveString(string: String) {
        print("Received: \(string)")
    }
}
