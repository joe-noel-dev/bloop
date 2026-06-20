package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import bloop.Bloop
import bloop.getRequest
import bloop.request
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.state.ConnectionType
import com.joenoel.bloop.state.ServerEndpoint
import com.joenoel.bloop.ui.theme.BloopTheme

@Composable
fun BloopApp(store: AppStoreViewModel) {
    val state by store.state.collectAsStateWithLifecycle()

    BloopAppContent(
        state = state,
        onStartCore = { store.dispatch(AppAction.ConnectLocal) },
        onDisconnect = { store.dispatch(AppAction.Disconnect) },
        onConnectRemoteHostPort = { host, port ->
            store.dispatch(AppAction.Connect(ServerEndpoint.HostPort(host, port)))
        },
        onConnectRemoteUrl = { url ->
            store.dispatch(AppAction.Connect(ServerEndpoint.Url(url)))
        },
        onRestartScan = { store.dispatch(AppAction.RestartScan) },
        onConnectDiscoveredServer = { endpoint ->
            store.dispatch(AppAction.Connect(endpoint))
        },
        onGetAll = {
            store.dispatch(
                AppAction.SendRequest(
                    request {
                        get = getRequest {
                            entity = Bloop.Entity.ALL
                        }
                    }
                )
            )
        }
    )
}

@Composable
private fun BloopAppContent(
    state: AppState,
    onStartCore: () -> Unit = {},
    onDisconnect: () -> Unit = {},
    onConnectRemoteHostPort: (String, Int) -> Unit = { _, _ -> },
    onConnectRemoteUrl: (String) -> Unit = {},
    onRestartScan: () -> Unit = {},
    onConnectDiscoveredServer: (ServerEndpoint) -> Unit = {},
    onGetAll: () -> Unit = {}
) {
    val scrollState = rememberScrollState()
    var remoteHost by rememberSaveable { mutableStateOf("127.0.0.1") }
    var remotePortText by rememberSaveable { mutableStateOf("14072") }
    var remoteUrl by rememberSaveable { mutableStateOf("ws://127.0.0.1:14072") }
    var remoteInputError by remember { mutableStateOf<String?>(null) }

    val isConnectedLocal = state.connected == ConnectionType.LOCAL
    val isConnectedRemote = state.connected == ConnectionType.REMOTE
    val getAllRequest = request {
        get = getRequest {
            entity = Bloop.Entity.ALL
        }
    }

    LaunchedEffect(Unit) {
        onRestartScan()
    }

    Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(MaterialTheme.colorScheme.background)
                .padding(horizontal = 24.dp, vertical = 32.dp)
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .align(Alignment.TopStart)
                    .verticalScroll(scrollState),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                Text(
                    text = androidx.compose.ui.res.stringResource(id = com.joenoel.bloop.R.string.app_name),
                    style = MaterialTheme.typography.displaySmall,
                    color = MaterialTheme.colorScheme.onBackground,
                    fontWeight = FontWeight.Bold
                )
                Text(
                    text = "State architecture ready",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.primary
                )
                Text(
                    text = "This shell now uses an app store with action/reducer/state flow modeled after iOS.",
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.84f)
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Current status",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Text(
                    text = "Embedded core: ${if (isConnectedLocal) "running" else "stopped"}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = if (isConnectedLocal) {
                        MaterialTheme.colorScheme.primary
                    } else {
                        MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                    }
                )
                Button(onClick = if (isConnectedLocal) onDisconnect else onStartCore) {
                    Text(if (isConnectedLocal) "Stop Local Core" else "Start Local Core")
                }
                Text(
                    text = "Remote server testing",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                OutlinedTextField(
                    value = remoteHost,
                    onValueChange = {
                        remoteHost = it
                        remoteInputError = null
                    },
                    label = { Text("Remote host") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth()
                )
                OutlinedTextField(
                    value = remotePortText,
                    onValueChange = {
                        remotePortText = it
                        remoteInputError = null
                    },
                    label = { Text("Remote port") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth()
                )
                Button(onClick = {
                    val host = remoteHost.trim()
                    val port = remotePortText.toIntOrNull()
                    if (host.isBlank() || port == null) {
                        remoteInputError = "Enter a valid host and numeric port"
                    } else {
                        onConnectRemoteHostPort(host, port)
                    }
                }) {
                    Text(if (isConnectedRemote) "Reconnect Remote (host:port)" else "Connect Remote (host:port)")
                }
                OutlinedTextField(
                    value = remoteUrl,
                    onValueChange = {
                        remoteUrl = it
                        remoteInputError = null
                    },
                    label = { Text("Remote ws URL") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth()
                )
                Button(onClick = {
                    val url = remoteUrl.trim()
                    if (url.isBlank()) {
                        remoteInputError = "Enter a valid ws:// URL"
                    } else {
                        onConnectRemoteUrl(url)
                    }
                }) {
                    Text("Connect Remote (ws URL)")
                }
                if (state.connected != null) {
                    Button(onClick = onDisconnect) {
                        Text("Disconnect")
                    }
                }
                if (remoteInputError != null) {
                    Text(
                        text = remoteInputError ?: "",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.error
                    )
                }
                Button(onClick = onGetAll) {
                    Text("Send Get All Request")
                }
                Text(
                    text = "Connection: ${connectionText(state.connected)}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Servers: ${state.servers.size} | Scanning: ${state.scanning}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Service discovery (temporary)",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Button(onClick = onRestartScan) {
                    Text("Restart Discovery")
                }
                if (state.scanning && state.servers.isEmpty()) {
                    Text(
                        text = "Scanning for _bloop._tcp services...",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                    )
                }
                if (!state.scanning && state.servers.isEmpty()) {
                    Text(
                        text = "No discovered services yet.",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                    )
                }
                state.servers.forEach { endpoint ->
                    when (endpoint) {
                        is ServerEndpoint.HostPort -> {
                            Button(onClick = { onConnectDiscoveredServer(endpoint) }) {
                                Text("Connect discovered ${endpoint.host}:${endpoint.port}")
                            }
                        }

                        is ServerEndpoint.Url -> {
                            Button(onClick = { onConnectDiscoveredServer(endpoint) }) {
                                Text("Connect discovered ${endpoint.value}")
                            }
                        }

                        is ServerEndpoint.Opaque -> {
                            Button(onClick = { onConnectDiscoveredServer(endpoint) }) {
                                Text("Connect discovered ${endpoint.value}")
                            }
                        }

                        is ServerEndpoint.Service -> {
                            Text(
                                text = "Discovered ${endpoint.name} (${endpoint.type})",
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                            )
                        }
                    }
                }
                Text(
                    text = "Projects: ${state.projects.size} local, ${state.cloudProjects.size} cloud",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Errors: ${state.errors.size} | Waveforms: ${state.waveforms.size}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Latest error: ${state.errors.lastOrNull() ?: "(none)"}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Latest response:",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Text(
                    text = state.lastResponseText ?: "(none)",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Serialized request",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Text(
                    text = getAllRequest.toString(),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Received projects",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Text(
                    text = serializedProjects(state.projects),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Received cloud projects",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.secondary
                )
                Text(
                    text = serializedProjects(state.cloudProjects),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
            }
        }
    }
}

private fun connectionText(connected: ConnectionType?): String {
    return when (connected) {
        ConnectionType.LOCAL -> "local"
        ConnectionType.REMOTE -> "remote"
        null -> "not connected"
    }
}

private fun serializedProjects(projects: List<Bloop.ProjectInfo>): String {
    return if (projects.isEmpty()) {
        "(none)"
    } else {
        projects.joinToString(separator = "\n\n") { project -> project.toString() }
    }
}

@Preview(showBackground = true)
@Composable
private fun BloopAppPreview() {
    BloopTheme {
        BloopAppContent(AppState())
    }
}
