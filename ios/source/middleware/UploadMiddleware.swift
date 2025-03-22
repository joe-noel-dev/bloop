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

        self.dispatch?(
            .sendRequest(
                .with {
                    $0.upload = .with {
                        $0.uploadID = uploadId
                        $0.data = slice
                    }
                }
            )
        )

        uploads[uploadId]?.position = end
    }

    private func completeUpload(uploadId: Id) {
        guard let upload = uploads[uploadId] else {
            return
        }

        self.dispatch?(
            .sendRequest(
                .with {
                    $0.completeUpload = .with {
                        $0.uploadID = uploadId
                    }
                }
            )
        )

        self.addSampleToSong(songId: upload.songId, uploadId: uploadId)

        uploads.removeValue(forKey: uploadId)
    }

    private func addSampleToSong(songId: Id, uploadId: Id) {
        self.dispatch?(
            .sendRequest(
                .with {
                    $0.addSample = .with {
                        $0.songID = songId
                        $0.uploadID = uploadId
                    }
                }
            )
        )
    }

    private func startUpload(songId: Id, file: URL) {
        do {
            let uploadId = randomId()
            let fileContents = try Data(contentsOf: file)
            uploads[uploadId] = Upload(data: fileContents, songId: songId)

            let filename = file.deletingPathExtension().lastPathComponent
            let fileExtension = file.pathExtension

            self.dispatch?(
                .sendRequest(
                    .with {
                        $0.beginUpload = .with {
                            $0.uploadID = uploadId
                            $0.filename = filename
                            $0.format = .wav
                        }
                    }
                )
            )
        }
        catch (let error) {
            print("Error loading audio file: \(error)")
        }
    }

}
