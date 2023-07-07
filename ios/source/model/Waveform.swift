import Foundation

struct WaveformData: Codable {
    var sampleRate: Int
    var peaks: [WaveformGroup]
}

struct WaveformGroup: Codable {
    var properties: WaveformProperties
    var values: [Float]
}

struct WaveformProperties: Codable {
    var length: Int
    var algorithm: WaveformAlgorithm
    var channel: Int
}

enum WaveformAlgorithm: String, Codable {
    case min
    case max
    case rms
}

typealias Waveforms = [Id: WaveformData]
