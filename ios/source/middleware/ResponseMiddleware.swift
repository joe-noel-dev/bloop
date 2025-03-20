import Foundation

class ResponseMiddleware: Middleware {
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {
        if case .receivedResponse(let response) = action {
            onResponse(response)
        }
    }

    private func onResponse(_ response: Response) {
        if let project = response.project {
            self.dispatch?(.setProject(project))
        }

        if let playback = response.playbackState {
            self.dispatch?(.setPlaybackState(playback))
        }

        if let progress = response.progress {
            self.dispatch?(.setProgress(progress))
        }

        if let projects = response.projects {
            self.dispatch?(.setProjects(projects))
        }

        if let error = response.error {
            self.dispatch?(.addError(error))
        }

        if let waveform = response.waveform {
            let action = Action.addWaveform((waveform.sampleId, waveform.waveformData))
            self.dispatch?(action)
        }

        if let uploadAck = response.upload {
            let action = Action.uploadAck(uploadAck.uploadId)
            self.dispatch?(action)
        }
        
        if let importResponse = response.importResponse {
            let action = Action.importResponse(importResponse)
            self.dispatch?(action)
        }
        
        if let exportResponse = response.exportResponse {
            let action = Action.exportResponse(exportResponse)
            self.dispatch?(action)
        }
    }
}
