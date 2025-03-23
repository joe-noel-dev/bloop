import Foundation

class ImportMiddleware: Middleware {
    var dispatch: Dispatch?

    let chunkSize = 1024 * 1024

    private struct Import {
        var data: Data
        var position: Int = 0
    }

    private var imports: [Id: Import] = [:]

    func execute(state: AppState, action: Action) {
        if case .importResponse(let importResponse) = action {
            sendChunk(importResponse.importID)
        }

        if case .importProject(let url) = action {
            beginImport(url)
        }

    }

    private func beginImport(_ url: URL) {
        do {
            let newProjectId = randomId()
            let fileContents = try Data(contentsOf: url)

            imports[newProjectId] = .init(data: fileContents)
            sendChunk(newProjectId)
        }
        catch (let error) {
            print("Error loading import file: \(error)")
        }
    }

    private func sendChunk(_ importId: Id) {
        guard let projectImport = imports[importId] else {
            return
        }

        let start = projectImport.position
        let end = min(projectImport.position + chunkSize, projectImport.data.count)
        let chunk = projectImport.data[start..<end]
        let moreComing = end != projectImport.data.count
        let progress = 100 * Double(end) / Double(projectImport.data.count)

        if moreComing {
            imports[importId]?.position = end
        }
        else {
            print("Project import complete for import: \(importId)")
            imports.removeValue(forKey: importId)
        }

        self.dispatch?(
            .sendRequest(
                .with {
                    $0.projectImport = .with {
                        $0.importID = importId
                        $0.data = chunk
                        $0.moreComing = moreComing
                    }
                }
            )
        )

        print("Import progress: \(progress)%")
    }

}
