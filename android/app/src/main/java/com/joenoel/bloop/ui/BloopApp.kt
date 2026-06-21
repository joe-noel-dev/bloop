package com.joenoel.bloop.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppStoreViewModel

@Composable
fun BloopApp(store: AppStoreViewModel) {
    val state by store.state.collectAsStateWithLifecycle()

    if (state.connected != null) {
        ConnectedPlaceholder()
    } else {
        ServerSelectionScreen(
            servers = state.servers,
            scanning = state.scanning,
            onLocalSelected = { store.dispatch(AppAction.ConnectLocal) },
            onServerSelected = { endpoint -> store.dispatch(AppAction.Connect(endpoint)) },
            onRestartScan = { store.dispatch(AppAction.RestartScan) },
        )
    }
}

@Composable
private fun ConnectedPlaceholder() {
    Surface(modifier = Modifier.fillMaxSize()) {
        Box(contentAlignment = Alignment.Center) {
            Text(
                text = "Connected",
                style = MaterialTheme.typography.headlineMedium,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}
