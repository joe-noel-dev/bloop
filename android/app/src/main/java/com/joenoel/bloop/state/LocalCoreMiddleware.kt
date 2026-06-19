package com.joenoel.bloop.state

import com.joenoel.bloop.core.BloopCore
import kotlinx.coroutines.CancellationException

fun interface LocalCoreFactory {
    fun create(onResponse: (ByteArray) -> Unit): LocalCoreSession
}

interface LocalCoreSession {
    fun sendRequest(request: ByteArray): Boolean
    fun close()
}

class LocalCoreMiddleware(
    private val bloopHome: String,
    private val coreFactory: LocalCoreFactory = LocalCoreFactory { onResponse ->
        BloopCoreSession(BloopCore(bloopHome, onResponse))
    }
) : AppMiddleware {

    private val lock = Any()
    private var core: LocalCoreSession? = null

    override suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit) {
        when (action) {
            AppAction.ConnectLocal -> connectLocal(dispatch)
            is AppAction.Connect -> {
                if (state.connected == ConnectionType.LOCAL) {
                    shutDownCore()
                }
            }
            AppAction.Disconnect -> {
                if (state.connected == ConnectionType.LOCAL) {
                    shutDownCore()
                    dispatch(AppAction.SetConnected(null))
                }
            }
            is AppAction.SendRawRequest -> {
                if (state.connected == ConnectionType.LOCAL) {
                    val accepted = synchronized(lock) {
                        core?.sendRequest(action.data) ?: false
                    }
                    if (!accepted) {
                        dispatch(AppAction.AddError("Failed to send request to local core"))
                    }
                }
            }
            else -> Unit
        }
    }

    private fun connectLocal(dispatch: (AppAction) -> Unit) {
        val started = synchronized(lock) {
            if (core != null) {
                true
            } else {
                try {
                    core = coreFactory.create { response ->
                        dispatch(AppAction.ReceivedRawResponse(response))
                    }
                    true
                } catch (error: CancellationException) {
                    throw error
                } catch (error: Throwable) {
                    dispatch(AppAction.AddError("Failed to start local core: ${error.message ?: "unknown error"}"))
                    false
                }
            }
        }

        if (started) {
            dispatch(AppAction.SetConnected(ConnectionType.LOCAL))
        }
    }

    private fun shutDownCore() {
        synchronized(lock) {
            core?.close()
            core = null
        }
    }
}

private class BloopCoreSession(private val core: BloopCore) : LocalCoreSession {
    override fun sendRequest(request: ByteArray): Boolean = core.sendRequest(request)

    override fun close() {
        core.close()
    }
}