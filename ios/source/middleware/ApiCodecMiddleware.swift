import SwiftBSON
import Foundation

class ApiCodecMiddleware: Middleware {
    var dispatch: Dispatch?
    
    func execute(state: AppState, action: Action) {
        if case .receivedRawResponse(let data) = action {
            onReceivedRawResponse(data)
        }
        
        if case .sendRequest(let request) = action {
            onSendRequest(request)
        }
    }
    
    private func onReceivedRawResponse(_ data: Data) {
        do {
            let bsonDocument = try BSONDocument(fromBSON: data)
            let response = try BSONDecoder().decode(Response.self, from: bsonDocument)
            self.dispatch?(.receivedResponse(response))
        }
        catch {
            print("Error decoding response from core: \(error)")
        }
    }
    
    private func onSendRequest(_ request: Request) {
        do {
            let encodedRequest = try BSONEncoder().encode(request)
            let data = encodedRequest.toData()
            self.dispatch?(.sendRawRequest(data))
        }
        catch {
            print("Error encoding request: \(error)")
        }
    }
    
}
