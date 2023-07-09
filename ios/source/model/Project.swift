import Foundation

struct Project: Codable, Equatable {
    var info: ProjectInfo
    var songs: [Song]
    var selections: Selections
}
