import Foundation
import SwiftBSON

class Core {
    var connection = CoreConnection()

    func sendRequest(_ request: Request) {
        do {
            let encodedRequest = try BSONEncoder().encode(request)
            let data = encodedRequest.toData()
            self.connection.send(data)
        } catch {
            print("Error encoding request: \(error)")
        }
    }
}
