import Foundation

extension Request {
    enum CodingKeys: String, CodingKey {
        case method
        case payload
    }

    enum RequestError: Error {
        case unknownMethod(String)
    }

    enum RequestMethod: String {
        case get
        case add
        case select
        case remove
        case update
        case duplicate
        case transport
        case save
        case load
        case rename
        case beginUpload
        case upload
        case completeUpload
        case addSample
        case removeSample
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        switch self {
        case .get(let entityId):
            try container.encode(RequestMethod.get.rawValue, forKey: .method)
            try container.encode(entityId, forKey: .payload)

        case .add(let entityId):
            try container.encode(RequestMethod.add.rawValue, forKey: .method)
            try container.encode(entityId, forKey: .payload)

        case .select(let entityId):
            try container.encode(RequestMethod.select.rawValue, forKey: .method)
            try container.encode(entityId, forKey: .payload)

        case .remove(let entityId):
            try container.encode(RequestMethod.remove.rawValue, forKey: .method)
            try container.encode(entityId, forKey: .payload)

        case .update(let updateRequest):
            try container.encode(RequestMethod.update.rawValue, forKey: .method)
            try container.encode(updateRequest, forKey: .payload)

        case .duplicate(let entityId):
            try container.encode(RequestMethod.duplicate.rawValue, forKey: .method)
            try container.encode(entityId, forKey: .payload)

        case .transport(let transportRequest):
            try container.encode(RequestMethod.transport.rawValue, forKey: .method)
            try container.encode(transportRequest, forKey: .payload)

        case .save:
            try container.encode(RequestMethod.save.rawValue, forKey: .method)

        case .load(let loadRequest):
            try container.encode(RequestMethod.load.rawValue, forKey: .method)
            try container.encode(loadRequest, forKey: .payload)

        case .rename(let renameRequest):
            try container.encode(RequestMethod.rename.rawValue, forKey: .method)
            try container.encode(renameRequest, forKey: .payload)

        case .beginUpload(let beginUploadRequest):
            try container.encode(RequestMethod.beginUpload.rawValue, forKey: .method)
            try container.encode(beginUploadRequest, forKey: .payload)

        case .upload(let uploadRequest):
            try container.encode(RequestMethod.upload.rawValue, forKey: .method)
            try container.encode(uploadRequest, forKey: .payload)

        case .completeUpload(let completeUploadRequest):
            try container.encode(RequestMethod.completeUpload.rawValue, forKey: .method)
            try container.encode(completeUploadRequest, forKey: .payload)

        case .addSample(let addSampleReqeust):
            try container.encode(RequestMethod.addSample.rawValue, forKey: .method)
            try container.encode(addSampleReqeust, forKey: .payload)

        case .removeSample(let removeSampleRequest):
            try container.encode(RequestMethod.removeSample.rawValue, forKey: .method)
            try container.encode(removeSampleRequest, forKey: .payload)
        }

    }

    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)

        let method = try values.decode(String.self, forKey: .method)

        if method == RequestMethod.get.rawValue {
            let payload = try values.decode(EntityId.self, forKey: .payload)
            self = .get(payload)
            return
        }

        if method == RequestMethod.add.rawValue {
            let payload = try values.decode(EntityId.self, forKey: .payload)
            self = .add(payload)
            return
        }

        if method == RequestMethod.select.rawValue {
            let payload = try values.decode(EntityId.self, forKey: .payload)
            self = .select(payload)
            return
        }

        if method == RequestMethod.remove.rawValue {
            let payload = try values.decode(EntityId.self, forKey: .payload)
            self = .remove(payload)
            return
        }

        if method == RequestMethod.update.rawValue {
            let payload = try values.decode(UpdateRequest.self, forKey: .payload)
            self = .update(payload)
            return
        }

        if method == RequestMethod.duplicate.rawValue {
            let payload = try values.decode(EntityId.self, forKey: .payload)
            self = .duplicate(payload)
            return
        }

        if method == RequestMethod.transport.rawValue {
            let payload = try values.decode(TransportRequest.self, forKey: .payload)
            self = .transport(payload)
            return
        }

        if method == RequestMethod.save.rawValue {
            self = .save
            return
        }

        if method == RequestMethod.load.rawValue {
            let payload = try values.decode(LoadRequest.self, forKey: .payload)
            self = .load(payload)
            return
        }

        if method == RequestMethod.rename.rawValue {
            let payload = try values.decode(RenameRequest.self, forKey: .payload)
            self = .rename(payload)
            return
        }

        if method == RequestMethod.beginUpload.rawValue {
            let payload = try values.decode(BeginUploadRequest.self, forKey: .payload)
            self = .beginUpload(payload)
            return
        }

        if method == RequestMethod.upload.rawValue {
            let payload = try values.decode(UploadRequest.self, forKey: .payload)
            self = .upload(payload)
            return
        }

        if method == RequestMethod.completeUpload.rawValue {
            let payload = try values.decode(CompleteUploadRequest.self, forKey: .payload)
            self = .completeUpload(payload)
            return
        }

        if method == RequestMethod.addSample.rawValue {
            let payload = try values.decode(AddSampleRequest.self, forKey: .payload)
            self = .addSample(payload)
            return
        }

        if method == RequestMethod.removeSample.rawValue {
            let payload = try values.decode(RemoveSampleRequest.self, forKey: .payload)
            self = .removeSample(payload)
            return
        }

        throw RequestError.unknownMethod(method)
    }
}

extension UpdateRequest {
    enum CodingKeys: String, CodingKey {
        case entity
        case value
    }

    enum Entity: String {
        case song
        case section
        case sample
    }

    enum UpdateRequestError: Error {
        case unknownEntity(String)
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        if case .song(let song) = self {
            try container.encode(Entity.song.rawValue, forKey: .entity)
            try container.encode(song, forKey: .value)
        }

        if case .section(let section) = self {
            try container.encode(Entity.section.rawValue, forKey: .entity)
            try container.encode(section, forKey: .value)
        }

        if case .sample(let sample) = self {
            try container.encode(Entity.sample.rawValue, forKey: .entity)
            try container.encode(sample, forKey: .value)
        }
    }

    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)

        let entity = try values.decode(String.self, forKey: .entity)

        if entity == Entity.song.rawValue {
            let value = try values.decode(Song.self, forKey: .value)
            self = .song(value)
            return
        }

        if entity == Entity.section.rawValue {
            let value = try values.decode(Section.self, forKey: .value)
            self = .section(value)
            return
        }

        if entity == Entity.sample.rawValue {
            let value = try values.decode(Sample.self, forKey: .value)
            self = .sample(value)
            return
        }

        throw UpdateRequestError.unknownEntity(entity)
    }
}

extension TransportRequest {
    enum CodingKeys: String, CodingKey {
        case method
        case options
    }

    enum TransportMethod: String {
        case play
        case stop
        case loop
        case exitLoop
        case queue
    }

    enum TransportRequestError: Error {
        case unknownMethod(String)
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        if case .play = self {
            try container.encode(TransportMethod.play.rawValue, forKey: .method)
        }

        if case .stop = self {
            try container.encode(TransportMethod.stop.rawValue, forKey: .method)
        }

        if case .loop = self {
            try container.encode(TransportMethod.loop.rawValue, forKey: .method)
        }

        if case .exitLoop = self {
            try container.encode(TransportMethod.exitLoop.rawValue, forKey: .method)
        }

        if case .queue(let queueRequest) = self {
            try container.encode(TransportMethod.queue.rawValue, forKey: .method)
            try container.encode(queueRequest, forKey: .options)
        }
    }

    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)

        let method = try values.decode(String.self, forKey: .method)

        if method == TransportMethod.play.rawValue {
            self = .play
            return
        }

        if method == TransportMethod.stop.rawValue {
            self = .stop
            return
        }

        if method == TransportMethod.loop.rawValue {
            self = .loop
            return
        }

        if method == TransportMethod.exitLoop.rawValue {
            self = .exitLoop
            return
        }

        if method == TransportMethod.queue.rawValue {
            let options = try values.decode(QueueRequest.self, forKey: .options)
            self = .queue(options)
            return
        }

        throw TransportRequestError.unknownMethod(method)
    }
}
