package com.joenoel.bloop.state

import bloop.Bloop

class AppCodecMiddleware : AppMiddleware {
    override suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit) {
        when (action) {
            is AppAction.SendRequest -> onSendRequest(action.request, dispatch)
            is AppAction.ReceivedRawResponse -> onReceivedRawResponse(action.data, dispatch)
            else -> Unit
        }
    }

    private fun onSendRequest(request: Bloop.Request, dispatch: (AppAction) -> Unit) {
try {
    dispatch(AppAction.SendRawRequest(request.toByteArray()))
} catch (error: Exception) {
    dispatch(AppAction.AddError("Failed to encode request: ${error.message ?: "unknown error"}"))
}
    }

    private fun onReceivedRawResponse(data: ByteArray, dispatch: (AppAction) -> Unit) {
        try {
            dispatch(AppAction.ReceivedResponse(Bloop.Response.parseFrom(data)))
        } catch (error: Throwable) {
            dispatch(AppAction.AddError("Failed to decode response: ${error.message ?: "unknown error"}"))
        }
    }
}