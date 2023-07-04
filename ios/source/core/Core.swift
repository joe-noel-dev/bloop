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

    init() {
        connection.delegate = self
    }

    func connect(_ ipAddress: String) {
        connection.connect(ipAddress)
    }

    func sendRequest(_ request: Request) {
        do {
            let encodedRequest = try BSONEncoder().encode(request)
            let data = encodedRequest.toData()
            self.connection.send(data)
        }
        catch {
            print("Error sending request: \(error)")
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
        do {
            let bsonDocument = try BSONDocument(fromBSON: data)
            let response = try BSONDecoder().decode(Response.self, from: bsonDocument)
            self.delegate?.coreDidSendResponse(response)
        }
        catch {
            print("Error decoding response from core: \(error)")
        }
    }

    func coreConnectionDidReceiveString(string: String) {
        print("Received: \(string)")
    }
}
