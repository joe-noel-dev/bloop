import Foundation

class ResponseMiddleware: Middleware {
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {
        if case .receivedResponse(let response) = action {
            onResponse(response)
        }
    }

    private func onResponse(_ response: Bloop_Response) {
        if response.hasProject {
            self.dispatch?(.setProject(response.project))
        }

        if response.hasPlaybackState {
            self.dispatch?(.setPlaybackState(response.playbackState))
        }

        if response.hasProgress {
            self.dispatch?(.setProgress(response.progress))
        }

        if !response.projects.isEmpty {
            self.dispatch?(.setProjects(response.projects))
        }

        if !response.error.isEmpty {
            self.dispatch?(.addError(response.error))
        }

        if response.hasWaveform {
            let waveform = response.waveform
            let action = Action.addWaveform((waveform.sampleID, waveform.waveformData))
            self.dispatch?(action)
        }

        if response.hasUpload {
            let action = Action.uploadAck(response.upload.uploadID)
            self.dispatch?(action)
        }

        if response.hasImportResponse {
            let action = Action.importResponse(response.importResponse)
            self.dispatch?(action)
        }

        if response.hasExportResponse {
            let action = Action.exportResponse(response.exportResponse)
            self.dispatch?(action)
        }
    }
}
