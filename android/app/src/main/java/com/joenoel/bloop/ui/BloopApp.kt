package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.state.ConnectionType
import com.joenoel.bloop.ui.theme.BloopTheme

@Composable
fun BloopApp(store: AppStoreViewModel) {
    val state by store.state.collectAsStateWithLifecycle()

    BloopAppContent(
        state = state,
        onStartCore = { store.dispatch(AppAction.ConnectLocal) },
        onStopCore = { store.dispatch(AppAction.Disconnect) }
    )
}

@Composable
private fun BloopAppContent(
    state: AppState,
    onStartCore: () -> Unit = {},
    onStopCore: () -> Unit = {}
) {
    val isCoreRunning = state.connected == ConnectionType.LOCAL

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
                    .align(Alignment.CenterStart),
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
                    text = "Embedded core: ${if (isCoreRunning) "running" else "stopped"}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = if (isCoreRunning) {
                        MaterialTheme.colorScheme.primary
                    } else {
                        MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                    }
                )
                Button(onClick = if (isCoreRunning) onStopCore else onStartCore) {
                    Text(if (isCoreRunning) "Stop Local Core" else "Start Local Core")
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
                    text = "Projects: ${state.projects.size} local, ${state.cloudProjects.size} cloud",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.72f)
                )
                Text(
                    text = "Errors: ${state.errors.size} | Waveforms: ${state.waveforms.size}",
                    style = MaterialTheme.typography.bodyMedium,
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

@Preview(showBackground = true)
@Composable
private fun BloopAppPreview() {
    BloopTheme {
        BloopAppContent(AppState())
    }
}
