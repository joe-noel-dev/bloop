package com.joenoel.bloop.state

import bloop.Bloop

enum class ConnectionType {
    LOCAL,
    REMOTE,
}

sealed interface ServerEndpoint {
    data class HostPort(val host: String, val port: Int) : ServerEndpoint
    data class Service(
        val name: String,
        val type: String,
        val domain: String,
        val interfaceName: String? = null,
    ) : ServerEndpoint
    data class Url(val value: String) : ServerEndpoint
    data class Opaque(val value: String) : ServerEndpoint
}

data class AppState(
    val connected: ConnectionType? = null,
    val scanning: Boolean = false,
    val servers: List<ServerEndpoint> = emptyList(),
    val projects: List<Bloop.ProjectInfo> = emptyList(),
    val cloudProjects: List<Bloop.ProjectInfo> = emptyList(),
    val project: Bloop.Project = emptyProject(),
    val projectInfo: Bloop.ProjectInfo? = null,
    val projectSyncStatuses: Map<String, Bloop.SyncStatus> = emptyMap(),
    val playbackState: Bloop.PlaybackState = Bloop.PlaybackState.getDefaultInstance(),
    val progress: Bloop.Progress = Bloop.Progress.getDefaultInstance(),
    val waveforms: Map<Long, Bloop.WaveformData> = emptyMap(),
    val user: Bloop.User? = null,
    val errors: List<String> = emptyList(),
    val preferences: Bloop.Preferences? = null,
    val lastResponseText: String? = null,
)

fun emptyProject(): Bloop.Project {
    return Bloop.Project
        .newBuilder()
        .setSelections(Bloop.Selections.getDefaultInstance())
        .build()
}
