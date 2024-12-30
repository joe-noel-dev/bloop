import Foundation
import SwiftBSON

protocol CoreDelegate: AnyObject {
    func coreConnected()
    func coreDisconnected()
    func coreDidSendResponse(_ response: Response)

    func onKnownServersChanged(_ servers: [Server])
}

class Core: CoreConnectionDelegate {
    private let connection = CoreConnection()
    weak var delegate: CoreDelegate?

    private let group = DispatchGroup()
    private let queue = DispatchQueue(label: "CoreEncodeDecode", attributes: .concurrent)
    private let discovery = Discovery()

    init() {
        connection.delegate = self

        discovery.onKnownServersChanged = { servers in
            DispatchQueue.main.async { [weak self] in
                self?.delegate?.onKnownServersChanged(servers)
            }
        }
    }

    func connect(_ server: Server) {
        if self.connection.state == .disconnected {
            self.connection.connect(server)
        }
    }

    func disconnect() {
        self.connection.disconnect()
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
