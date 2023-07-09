import Foundation

struct ProjectInfo: Codable, Equatable, Identifiable {
    var id: Id
    var name: String
    var version: String
    var lastSaved: Int64
}
