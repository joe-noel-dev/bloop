import Foundation

class UploadMiddleware: Middleware {

    struct Upload {
        var data: Data
        var songId: Id
        var position: Int = 0
    }

    static private let chunkSize = 1024 * 1024

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

    private func copyFileLocally(url: URL) -> URL? {
        guard url.startAccessingSecurityScopedResource() else {
            print("Failed to start security-scoped access.")
            return nil
        }
        defer { url.stopAccessingSecurityScopedResource() }

        let tempDirectory = FileManager.default.temporaryDirectory
        let tempURL = tempDirectory.appendingPathComponent(url.lastPathComponent)

        do {
            if FileManager.default.fileExists(atPath: tempURL.path) {
                try FileManager.default.removeItem(at: tempURL)
            }
            try FileManager.default.copyItem(at: url, to: tempURL)
            print("File copied to temporary location: \(tempURL)")
        }
        catch {
            print("Failed to copy file to temporary location: \(error)")
            return nil
        }

        return tempURL
    }

    private func startUpload(songId: Id, file: URL) {
        guard let localURL = copyFileLocally(url: file) else {
            return
        }

        do {
            let uploadId = randomId()
            let fileContents = try Data(contentsOf: localURL)
            uploads[uploadId] = Upload(data: fileContents, songId: songId)

            let filename = localURL.deletingPathExtension().lastPathComponent

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
