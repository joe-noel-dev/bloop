import Foundation

struct ProjectInfo: Codable {
    var id: Id
    var name: String
    var version: String
    var lastSaved: Int64
}
