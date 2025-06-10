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

    case .setProjectInfo(let projectInfo):
        state.projectInfo = projectInfo

    case .addError(let error):
        print("Error from core: \(error)")
        state.errors.append(error)

    case .addWaveform((let id, let waveform)):
        state.waveforms[id] = waveform

    case .removeWaveform(let id):
        state.waveforms.removeValue(forKey: id)

    case .setDiscoveredServers(let servers):
        state.servers = servers

    case .setScanning(let scanning):
        state.scanning = scanning

    case .removeAllServers:
        state.servers.removeAll()

    case .setUser(let user):
        state.user = user

    case .clearUser:
        state.user = nil

    default:
        break
    }

    return state
}
