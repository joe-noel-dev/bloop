import Foundation

struct Project: Codable {
    var info: ProjectInfo
    var songs: [Song]
    var selections: Selections
}
