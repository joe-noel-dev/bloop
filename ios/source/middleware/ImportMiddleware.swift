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
            sendChunk(importResponse.projectID)
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
    
    private func sendChunk(_ projectId: Id) {
        guard let projectImport = imports[projectId] else {
            return
        }
        
        let start = projectImport.position
        let end = min(projectImport.position + chunkSize, projectImport.data.count)
        let chunk = projectImport.data[start..<end]
        let moreComing = end != projectImport.data.count
        let progress = 100 * Double(end) / Double(projectImport.data.count)
        
        if moreComing {
            imports[projectId]?.position = end
        } else {
            print("Project import complete for project: \(projectId)")
            imports.removeValue(forKey: projectId)
        }
    
        self.dispatch?(.sendRequest(.with {
            $0.projectImport = .with {
                $0.projectID = projectId
                $0.data = chunk
                $0.moreComing = moreComing
            }
        }))
        
        print("Import progress: \(progress)%")
    }

}
