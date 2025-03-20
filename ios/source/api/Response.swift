import Foundation

struct Response: Codable {
    var project: Project?
    var projects: [ProjectInfo]?
    var playbackState: PlaybackState?
    var waveform: WaveformResponse?
    var progress: Progress?
    var upload: UploadAck?
    var error: String?
    var exportResponse: ExportResponse?
    var importResponse: ImportResponse?
}

struct WaveformResponse: Codable {
    var sampleId: Id
    var waveformData: WaveformData
}

struct UploadAck: Codable {
    var uploadId: Id
}

struct ExportResponse: Codable {
    var projectId: Id
    var data: Data
    var moreComing: Bool
}

struct ImportResponse: Codable {
    var projectId: Id
}
