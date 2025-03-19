import Foundation
import SwiftBSON

class ApiCodecMiddleware: Middleware {
    var dispatch: Dispatch?

    var queue = DispatchQueue(label: "ApiCodecMiddleware")

    func execute(state: AppState, action: Action) {
        if case .receivedRawResponse(let data) = action {
            onReceivedRawResponse(data)
        }

        if case .sendRequest(let request) = action {
            onSendRequest(request)
        }
    }

    private func onReceivedRawResponse(_ data: Data) {
        queue.async { [weak self] in
            do {
                let bsonDocument = try BSONDocument(fromBSON: data)
                let response = try BSONDecoder().decode(Response.self, from: bsonDocument)
                DispatchQueue.main.async {
                    self?.dispatch?(.receivedResponse(response))
                }
            }
            catch {
                print("Error decoding response from core: \(error)")
            }
        }
    }

    private func onSendRequest(_ request: Request) {
        queue.async { [weak self] in
            do {
                let encodedRequest = try BSONEncoder().encode(request)
                let data = encodedRequest.toData()
                DispatchQueue.main.async {
                    self?.dispatch?(.sendRawRequest(data))
                }
            }
            catch {
                print("Error encoding request: \(error)")
            }
        }
    }

}
