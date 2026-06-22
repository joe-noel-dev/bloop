package com.joenoel.bloop.ui

import android.app.Activity
import android.view.WindowManager
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppStoreViewModel

@Composable
fun BloopApp(store: AppStoreViewModel) {
    val state by store.state.collectAsStateWithLifecycle()

    if (state.connected != null) {
        KeepScreenOn()
        ProjectScreen(
            state = state,
            onDispatch = store::dispatch,
        )
    } else {
        LaunchedEffect(Unit) {
            store.dispatch(AppAction.RestartScan)
        }
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
private fun KeepScreenOn() {
    val context = LocalContext.current
    DisposableEffect(Unit) {
        val window = (context as Activity).window
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        onDispose {
            window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        }
    }
}
