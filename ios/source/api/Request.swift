import Foundation

enum Request: Codable {
    case get(EntityId)
    case add(EntityId)
    case select(EntityId)
    case remove(EntityId)
    case update(UpdateRequest)
    case transport(TransportRequest)
    case save
    case load(LoadRequest)
    case rename(RenameRequest)
    case beginUpload(BeginUploadRequest)
    case upload(UploadRequest)
    case completeUpload(CompleteUploadRequest)
    case addSample(AddSampleRequest)
    case removeSample(RemoveSampleRequest)
}

struct EntityId: Codable {
    var entity: Entity
    var id: Id?
}

enum Entity: Codable {
    case all
    case project
    case projects
    case sample
    case section
    case song
    case waveform
}

enum UpdateRequest: Codable {
    case song(Song)
    case section(Section)
    case sample(Sample)
}

enum TransportRequest: Codable {
    case play
    case stop
    case loop
    case exitLoop
    case queue(QueueRequest)
}

struct LoadRequest: Codable {
    var id: Id
}

struct RenameRequest: Codable {
    var entity: Entity
    var id: Id?
    var name: String
}

struct BeginUploadRequest: Codable {
    var uploadId: Id
    var filename: String
    var format: String
}

struct UploadRequest: Codable {
    var uploadId: Id
    var data: Data
}

struct CompleteUploadRequest: Codable {
    var uploadId: Id
}

struct AddSampleRequest: Codable {
    var songId: Id
    var uploadId: Id
}

struct RemoveSampleRequest: Codable {
    var songId: Id
}

struct QueueRequest: Codable {
    var songId: Id
    var sectionId: Id
}
