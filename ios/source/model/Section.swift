import Foundation

struct Section: Codable, Equatable {
    var id: Id
    var name: String
    var start: Double
    var loop: Bool
    var metronome: Bool
}
