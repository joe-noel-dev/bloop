import Foundation

struct Server: Identifiable, Equatable {
    var hostname: String
    var port: Int

    var id: String {
        return "\(hostname):\(port)"
    }
}
