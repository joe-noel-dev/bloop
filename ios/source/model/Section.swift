import Foundation

struct Section: Codable {
    var id: Id
    var name: String
    var start: Double
    var loop: Bool
    var metronome: Bool
}
