import Foundation

struct ProjectInfo: Codable, Identifiable {
    var id: Id
    var name: String
    var version: String
    var lastSaved: Int64
}
