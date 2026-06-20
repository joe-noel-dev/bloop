package com.joenoel.bloop.state

import bloop.Bloop
import bloop.getRequest
import bloop.request
import com.joenoel.bloop.network.OkHttpRemoteConnection
import com.joenoel.bloop.network.RemoteConnection
import com.joenoel.bloop.network.RemoteConnectionFactory
import com.joenoel.bloop.network.RemoteConnectionListener

class RemoteMiddleware(
    private val connectionFactory: RemoteConnectionFactory = RemoteConnectionFactory { url, listener ->
        OkHttpRemoteConnection(url, listener)
    },
) : AppMiddleware {

    private val lock = Any()
    private var connection: RemoteConnection? = null

    override suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit) {
        when (action) {
            is AppAction.Connect -> connectRemote(action.server, dispatch)
            AppAction.ConnectLocal -> {
                if (state.connected == ConnectionType.REMOTE) {
                    disconnectRemote()
                }
            }
            AppAction.Disconnect -> {
                if (state.connected == ConnectionType.REMOTE) {
                    disconnectRemote()
                    dispatch(AppAction.SetConnected(null))
                }
            }
            is AppAction.SendRawRequest -> {
                if (state.connected == ConnectionType.REMOTE) {
                    val sent = synchronized(lock) { connection?.send(action.data) ?: false }
                    if (!sent) {
                        dispatch(AppAction.AddError("Failed to send request to remote server"))
                    }
                }
            }
            else -> Unit
        }
    }

    private fun connectRemote(server: ServerEndpoint, dispatch: (AppAction) -> Unit) {
        val url = urlFromEndpoint(server) ?: run {
            dispatch(AppAction.AddError("Cannot connect: unsupported endpoint type $server"))
            return
        }

        disconnectRemote()

        var createError: Throwable? = null

        synchronized(lock) {
            var createdConnection: RemoteConnection? = null

            val listener = object : RemoteConnectionListener {
                private fun isCurrent(): Boolean = synchronized(lock) { connection === createdConnection }

                override fun onConnected() {
                    if (!isCurrent()) return
                    dispatch(AppAction.SetConnected(ConnectionType.REMOTE))
                    dispatch(
                        AppAction.SendRequest(
                            request { get = getRequest { entity = Bloop.Entity.ALL } }
                        )
                    )
                }

                override fun onDisconnected() {
                    if (!isCurrent()) return
                    dispatch(AppAction.SetConnected(null))
                }

                override fun onMessage(data: ByteArray) {
                    if (!isCurrent()) return
                    dispatch(AppAction.ReceivedRawResponse(data))
                }
            }

            createdConnection = try {
                connectionFactory.create(url, listener)
            } catch (t: Throwable) {
                createError = t
                null
            }

            connection = createdConnection
        }

        if (createError != null) {
            dispatch(
                AppAction.AddError(
                    "Cannot connect to remote server at $url: ${createError?.message ?: "unknown error"}"
                )
            )
        }
    }

    private fun disconnectRemote() {
        synchronized(lock) {
            connection?.disconnect()
            connection = null
        }
    }

    private fun urlFromEndpoint(server: ServerEndpoint): String? = when (server) {
        is ServerEndpoint.Url -> server.value
        is ServerEndpoint.HostPort -> "ws://${server.host}:${server.port}"
        is ServerEndpoint.Opaque -> server.value
        is ServerEndpoint.Service -> null
    }
}
