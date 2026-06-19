package com.joenoel.bloop.state

object AppReducer {
    fun reduce(state: AppState, action: AppAction): AppState {
        return when (action) {
            is AppAction.SetProject -> state.copy(project = action.project)
            is AppAction.SetPlaybackState -> state.copy(playbackState = action.playbackState)
            is AppAction.SetProgress -> state.copy(progress = action.progress)
            is AppAction.SetProjects -> state.copy(projects = action.projects)
            is AppAction.SetCloudProjects -> state.copy(cloudProjects = action.cloudProjects)
            is AppAction.SetProjectInfo -> state.copy(projectInfo = action.projectInfo)
            is AppAction.SetProjectSync -> state.copy(
                projectSyncStatuses = state.projectSyncStatuses +
                    (action.projectId to action.syncState)
            )
            is AppAction.DismissProjectSync -> state.copy(
                projectSyncStatuses = state.projectSyncStatuses - action.projectId
            )
            is AppAction.AddError -> state.copy(errors = state.errors + action.error)
            is AppAction.AddWaveform -> state.copy(waveforms = state.waveforms + (action.id to action.waveform))
            is AppAction.RemoveWaveform -> state.copy(waveforms = state.waveforms - action.id)
            is AppAction.SetDiscoveredServers -> state.copy(servers = action.servers)
            is AppAction.SetScanning -> state.copy(scanning = action.scanning)
            AppAction.RemoveAllServers -> state.copy(servers = emptyList())
            is AppAction.SetUser -> state.copy(user = action.user)
            AppAction.ClearUser -> state.copy(user = null)
            is AppAction.SetPreferences -> state.copy(preferences = action.preferences)
            is AppAction.SetConnected -> state.copy(connected = action.connected)
            else -> state
        }
    }
}
