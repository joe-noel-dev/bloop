package com.joenoel.bloop.network

import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import okio.ByteString
import okio.ByteString.Companion.toByteString
import java.util.concurrent.TimeUnit

interface RemoteConnectionListener {
    fun onConnected()
    fun onDisconnected()
    fun onMessage(data: ByteArray)
}

fun interface RemoteConnectionFactory {
    fun create(url: String, listener: RemoteConnectionListener): RemoteConnection
}

interface RemoteConnection {
    fun send(data: ByteArray): Boolean
    fun disconnect()
}

internal class OkHttpRemoteConnection(
    url: String,
    private val listener: RemoteConnectionListener,
) : RemoteConnection, WebSocketListener() {

    private val client = OkHttpClient.Builder()
        .pingInterval(30, TimeUnit.SECONDS)
        .build()

    private val webSocket: WebSocket = client.newWebSocket(
        Request.Builder().url(url).build(),
        this,
    )

    override fun send(data: ByteArray): Boolean = webSocket.send(data.toByteString())

    override fun disconnect() {
        webSocket.close(1000, null)
        client.dispatcher.executorService.shutdown()
    }

    override fun onOpen(webSocket: WebSocket, response: Response) {
        listener.onConnected()
    }

    override fun onMessage(webSocket: WebSocket, bytes: ByteString) {
        listener.onMessage(bytes.toByteArray())
    }

    override fun onClosed(webSocket: WebSocket, code: Int, reason: String) {
        listener.onDisconnected()
        client.dispatcher.executorService.shutdown()
    }

    override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
        listener.onDisconnected()
        client.dispatcher.executorService.shutdown()
    }
}
