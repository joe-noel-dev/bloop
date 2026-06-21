package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Computer
import androidx.compose.material.icons.outlined.WifiOff
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.joenoel.bloop.BuildConfig
import com.joenoel.bloop.state.ServerEndpoint
import com.joenoel.bloop.ui.theme.BloopTheme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ServerSelectionScreen(
    servers: List<ServerEndpoint>,
    scanning: Boolean,
    onLocalSelected: () -> Unit,
    onServerSelected: (ServerEndpoint) -> Unit,
    onRestartScan: (() -> Unit)? = null,
    onCancel: (() -> Unit)? = null,
) {
    val scrollState = rememberScrollState()
    Scaffold(
        topBar = {
            if (onCancel != null) {
                TopAppBar(
                    title = {
                        Text(
                            text = "Select Server",
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.SemiBold,
                        )
                    },
                    navigationIcon = {
                        TextButton(onClick = onCancel) {
                            Text("Cancel")
                        }
                    }
                )
            }
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
                .padding(horizontal = 24.dp)
                .verticalScroll(scrollState),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            if (onCancel == null) {
                Spacer(modifier = Modifier.height(80.dp))
            }

            Button(
                onClick = onLocalSelected,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 8.dp),
            ) {
                Icon(
                    imageVector = Icons.Outlined.Computer,
                    contentDescription = null,
                    modifier = Modifier.padding(end = 8.dp),
                )
                Text(
                    text = if (onCancel != null) "Start Local" else "Start",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            if (onCancel != null) {
                Text(
                    text = "Run Bloop locally on this device",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 4.dp),
                )
            }

            if (servers.isEmpty() && scanning) {
                Column(
                    modifier = Modifier.padding(top = 32.dp),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                ) {
                    CircularProgressIndicator()
                    Text(
                        text = "Scanning for servers...",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }

            if (servers.isEmpty() && !scanning) {
                Column(
                    modifier = Modifier.padding(top = 32.dp, start = 24.dp, end = 24.dp),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    Icon(
                        imageVector = Icons.Outlined.WifiOff,
                        contentDescription = null,
                        modifier = Modifier.size(48.dp),
                        tint = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                    Text(
                        text = "No servers found",
                        style = MaterialTheme.typography.headlineSmall,
                    )
                    Text(
                        text = "Make sure your device is connected to the same network as the Bloop server.",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        textAlign = TextAlign.Center,
                    )
                }
            }

            if (servers.isNotEmpty()) {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(top = 24.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    Text(
                        text = "Available Servers",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold,
                        modifier = Modifier.padding(horizontal = 8.dp),
                    )
                    servers.forEach { server ->
                        ServerRow(
                            server = server,
                            onConnect = { onServerSelected(server) },
                        )
                    }
                }
            }

            Spacer(modifier = Modifier.height(80.dp))

            if (onRestartScan != null) {
                TextButton(
                    onClick = onRestartScan,
                    modifier = Modifier.padding(bottom = 4.dp),
                ) {
                    Text(
                        text = "Restart scan",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }

            Text(
                text = versionName(),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.6f),
                modifier = Modifier.padding(bottom = 16.dp),
            )
        }
    }
}

@Composable
private fun ServerRow(
    server: ServerEndpoint,
    onConnect: () -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(
                color = MaterialTheme.colorScheme.surfaceVariant,
                shape = MaterialTheme.shapes.medium,
            )
            .padding(16.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = displayName(server),
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.weight(1f),
        )
        OutlinedButton(onClick = onConnect) {
            Text(text = "Connect", fontWeight = FontWeight.Medium)
        }
    }
}

private fun displayName(endpoint: ServerEndpoint): String = when (endpoint) {
    is ServerEndpoint.HostPort -> endpoint.displayName ?: endpoint.host
    is ServerEndpoint.Service -> endpoint.name
    is ServerEndpoint.Url -> endpoint.value
    is ServerEndpoint.Opaque -> endpoint.value
}

private fun versionName(): String = BuildConfig.VERSION_NAME.ifBlank { "0.0.0" }

@Preview(showBackground = true)
@Composable
private fun ServerSelectionScreenPreview_Scanning() {
    BloopTheme {
        ServerSelectionScreen(
            servers = emptyList(),
            scanning = true,
            onLocalSelected = {},
            onServerSelected = {},
            onRestartScan = {},
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun ServerSelectionScreenPreview_NoServers() {
    BloopTheme {
        ServerSelectionScreen(
            servers = emptyList(),
            scanning = false,
            onLocalSelected = {},
            onServerSelected = {},
            onRestartScan = {},
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun ServerSelectionScreenPreview_WithServers() {
    BloopTheme {
        ServerSelectionScreen(
            servers = listOf(
                ServerEndpoint.HostPort("192.168.1.100", 14072),
                ServerEndpoint.Service("Bloop-Living-Room", "_bloop._tcp", "local"),
            ),
            scanning = false,
            onLocalSelected = {},
            onServerSelected = {},
            onRestartScan = {},
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun ServerSelectionScreenPreview_SheetMode() {
    BloopTheme {
        ServerSelectionScreen(
            servers = listOf(ServerEndpoint.HostPort("192.168.1.5", 14072)),
            scanning = false,
            onLocalSelected = {},
            onServerSelected = {},
            onRestartScan = {},
            onCancel = {},
        )
    }
}
