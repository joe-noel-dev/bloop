package com.joenoel.bloop.state

import bloop.Bloop

sealed interface AppAction {
    data class SendRequest(val request: Bloop.Request) : AppAction
    data class ReceivedResponse(val response: Bloop.Response) : AppAction

    data class SendRawRequest(val data: ByteArray) : AppAction
    data class ReceivedRawResponse(val data: ByteArray) : AppAction

    data class SetProject(val project: Bloop.Project) : AppAction
    data class SetPlaybackState(val playbackState: Bloop.PlaybackState) : AppAction
    data class SetProgress(val progress: Bloop.Progress) : AppAction
    data class SetProjects(val projects: List<Bloop.ProjectInfo>) : AppAction
    data class SetCloudProjects(val cloudProjects: List<Bloop.ProjectInfo>) : AppAction
    data class SetProjectInfo(val projectInfo: Bloop.ProjectInfo) : AppAction
    data class SetProjectSync(val projectId: String, val syncState: Bloop.SyncStatus) : AppAction
    data class DismissProjectSync(val projectId: String) : AppAction
    data class AddWaveform(val id: Long, val waveform: Bloop.WaveformData) : AppAction
    data class RemoveWaveform(val id: Long) : AppAction
    data class SetUser(val user: Bloop.User) : AppAction
    data object ClearUser : AppAction
    data class AddError(val error: String) : AppAction
    data class SetPreferences(val preferences: Bloop.Preferences) : AppAction
    data class SetLastResponseText(val text: String) : AppAction

    data class Connect(val server: ServerEndpoint) : AppAction
    data object ConnectLocal : AppAction
    data object Disconnect : AppAction
    data class SetConnected(val connected: ConnectionType?) : AppAction

    data class UploadSample(val songId: Long, val fileUri: String) : AppAction
    data class UploadAck(val uploadId: Long) : AppAction

    data class SetDiscoveredServers(val servers: List<ServerEndpoint>) : AppAction
    data class SetScanning(val scanning: Boolean) : AppAction
    data object RemoveAllServers : AppAction

    data object RestartScan : AppAction
    data object StopScan : AppAction
}
