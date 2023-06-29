import Foundation

struct Sample: Codable {
    var id: Id
    var name: String
    var tempo: Tempo
    var sampleRate: Double
    var sampleCount: Int
    var channelCount: Int
}
