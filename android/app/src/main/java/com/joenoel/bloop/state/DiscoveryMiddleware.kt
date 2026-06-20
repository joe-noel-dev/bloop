package com.joenoel.bloop.state

class DiscoveryMiddleware(
    private val discoveryController: ServiceDiscoveryController,
) : AppMiddleware {

    override suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit) {
        when (action) {
            AppAction.RestartScan -> {
                dispatch(AppAction.RemoveAllServers)
                discoveryController.restart(
                    onScanningChanged = { scanning ->
                        dispatch(AppAction.SetScanning(scanning))
                    },
                    onServersChanged = { servers ->
                        dispatch(AppAction.SetDiscoveredServers(servers))
                    },
                    onError = { message ->
                        dispatch(AppAction.AddError(message))
                    }
                )
            }
            else -> Unit
        }
    }
}
