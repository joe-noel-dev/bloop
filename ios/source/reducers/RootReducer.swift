import Foundation

func rootReducer(state: AppState, action: Action) -> AppState {
    var state = state

    switch action {
    case .setProject(let project):
        state.project = project

    case .setPlaybackState(let playbackState):
        state.playbackState = playbackState

    case .setConnected(let connected):
        state.connected = connected

    case .setProgress(let progress):
        state.progress = progress

    case .setProjects(let projects):
        state.projects = projects

    case .addError(let error):
        print("Error from core: \(error)")

    case .addWaveform((let id, let waveform)):
        state.waveforms[id] = waveform

    case .removeWaveform(let id):
        state.waveforms.removeValue(forKey: id)

    case .connect(_):
        break

    case .uploadSample(_):
        break

    case .uploadAck(_):
        break

    case .setDiscoveredServers(let servers):
        state.servers = servers

    case .removeAllServers:
        state.servers.removeAll()

    case .sendRequest(_):
        break
    }

    return state
}
