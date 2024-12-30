import Foundation

class UploadMiddleware: Middleware {

    struct Upload {
        var data: Data
        var songId: Id
        var position: Int = 0
    }

    static private let chunkSize = 10 * 1024

    private var uploads: [Id: Upload] = [:]
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {
        if case .uploadSample((let songId, let fileToUpload)) = action {
            startUpload(songId: songId, file: fileToUpload)
        }

        if case .uploadAck(let uploadId) = action {
            onUploadAck(uploadId: uploadId)
        }
    }

    func setDispatch(_ dispatch: @escaping Dispatch) {
        self.dispatch = dispatch
    }

    private func onUploadAck(uploadId: Id) {
        guard let upload = uploads[uploadId] else {
            return
        }

        let start = upload.position
        let end = min(upload.position + Self.chunkSize, upload.data.count)

        if start == end {
            completeUpload(uploadId: uploadId)
            return
        }

        let slice = upload.data.subdata(in: start..<end)

        let uploadRequest = UploadRequest(uploadId: uploadId, data: slice)
        let request = Request.upload(uploadRequest)
        let action = Action.sendRequest(request)
        self.dispatch?(action)

        uploads[uploadId]?.position = end
    }

    private func completeUpload(uploadId: Id) {
        guard let upload = uploads[uploadId] else {
            return
        }

        let completeRequest = CompleteUploadRequest(uploadId: uploadId)
        let request = Request.completeUpload(completeRequest)
        let action = Action.sendRequest(request)
        self.dispatch?(action)

        self.addSampleToSong(songId: upload.songId, uploadId: uploadId)

        uploads.removeValue(forKey: uploadId)
    }

    private func addSampleToSong(songId: Id, uploadId: Id) {
        let addSampleRequest = AddSampleRequest(songId: songId, uploadId: uploadId)
        let request = Request.addSample(addSampleRequest)
        let action = Action.sendRequest(request)
        self.dispatch?(action)
    }

    private func startUpload(songId: Id, file: URL) {
        do {
            let uploadId = UUID().uuidString.lowercased()
            let fileContents = try Data(contentsOf: file)
            uploads[uploadId] = Upload(data: fileContents, songId: songId)

            let filename = file.deletingPathExtension().lastPathComponent
            let fileExtension = file.pathExtension

            let beginRequest = BeginUploadRequest(
                uploadId: uploadId,
                filename: filename,
                format: fileExtension
            )
            let request = Request.beginUpload(beginRequest)
            let action = Action.sendRequest(request)
            self.dispatch?(action)
        }
        catch (let error) {
            print("Error loading audio file: \(error)")
        }
    }

}
