import Foundation

class ApiMiddleware: Middleware {
    var dispatch: Dispatch?
    private let core = Core()

    init() {
        core.delegate = self
    }

    func execute(state: AppState, action: Action) {
        if case .connect(let server) = action {
            core.connect(server)
        }

        if case .sendRequest(let request) = action {
            core.sendRequest(request)
        }
    }
    

}

extension ApiMiddleware: CoreDelegate {
    func coreConnected() {
        self.dispatch?(.setConnected(true))
    }

    func coreDisconnected() {
        self.dispatch?(.setConnected(false))
    }

    func coreDidSendResponse(_ response: Response) {
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
    }

    func onKnownServersChanged(_ servers: [Server]) {
        self.dispatch?(.setDiscoveredServers(servers))
    }

}
