package com.joenoel.bloop.state

import com.joenoel.bloop.network.RemoteConnection
import com.joenoel.bloop.network.RemoteConnectionFactory
import com.joenoel.bloop.network.RemoteConnectionListener
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class RemoteMiddlewareTest {

    @Test
    fun `connect dispatches SetConnected remote when WebSocket opens`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { dispatched += it }
        factory.lastListener?.onConnected()

        assertTrue(dispatched.contains(AppAction.SetConnected(ConnectionType.REMOTE)))
    }

    @Test
    fun `connect sends GetAll request after connecting`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { dispatched += it }
        factory.lastListener?.onConnected()

        assertTrue(dispatched.any { it is AppAction.SendRequest })
    }

    @Test
    fun `disconnect while remote closes connection and clears state`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { dispatched += it }
        middleware.execute(AppState(connected = ConnectionType.REMOTE), AppAction.Disconnect) { dispatched += it }

        assertEquals(1, factory.lastConnection?.disconnectCount)
        assertTrue(dispatched.contains(AppAction.SetConnected(null)))
    }

    @Test
    fun `disconnect while not remote does not close remote connection`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { }
        middleware.execute(AppState(connected = ConnectionType.LOCAL), AppAction.Disconnect) { dispatched += it }

        assertEquals(0, factory.lastConnection?.disconnectCount ?: 0)
        assertFalse(dispatched.contains(AppAction.SetConnected(null)))
    }

    @Test
    fun `send raw request forwards data to WebSocket`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val payload = byteArrayOf(1, 2, 3)

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { }
        middleware.execute(AppState(connected = ConnectionType.REMOTE), AppAction.SendRawRequest(payload)) { }

        val sent = factory.lastConnection?.sentData?.firstOrNull()
        assertNotNull(sent)
        assertTrue(payload.contentEquals(sent!!))
    }

    @Test
    fun `send raw request while not remote is ignored`() = runTest {
        val (middleware, factory) = middlewareWithFactory()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { }
        middleware.execute(AppState(connected = ConnectionType.LOCAL), AppAction.SendRawRequest(byteArrayOf(1))) { }

        assertEquals(0, factory.lastConnection?.sentData?.size ?: 0)
    }

    @Test
    fun `failed send emits error action`() = runTest {
        val (middleware, _) = middlewareWithFactory(sendResult = false)
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { }
        middleware.execute(AppState(connected = ConnectionType.REMOTE), AppAction.SendRawRequest(byteArrayOf(9))) { dispatched += it }

        assertTrue(dispatched.any { it is AppAction.AddError })
    }

    @Test
    fun `WebSocket message dispatches ReceivedRawResponse`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()
        val response = byteArrayOf(5, 6, 7)

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { dispatched += it }
        factory.lastListener?.onMessage(response)

        val rawResponses = dispatched.filterIsInstance<AppAction.ReceivedRawResponse>()
        assertEquals(1, rawResponses.size)
        assertTrue(response.contentEquals(rawResponses.first().data))
    }

    @Test
    fun `WebSocket disconnect dispatches SetConnected null`() = runTest {
        val (middleware, factory) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { dispatched += it }
        factory.lastListener?.onDisconnected()

        assertTrue(dispatched.contains(AppAction.SetConnected(null)))
    }

    @Test
    fun `HostPort endpoint builds ws URL`() = runTest {
        val (middleware, factory) = middlewareWithFactory()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.HostPort("192.168.1.1", 14072))) { }

        assertEquals("ws://192.168.1.1:14072", factory.lastUrl)
    }

    @Test
    fun `Opaque endpoint uses value as URL`() = runTest {
        val (middleware, factory) = middlewareWithFactory()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Opaque("ws://opaque:1234"))) { }

        assertEquals("ws://opaque:1234", factory.lastUrl)
    }

    @Test
    fun `Service endpoint emits unsupported error`() = runTest {
        val (middleware, _) = middlewareWithFactory()
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(
            AppState(),
            AppAction.Connect(ServerEndpoint.Service("bloop", "_bloop._tcp", "local.")),
        ) { dispatched += it }

        assertTrue(dispatched.any { it is AppAction.AddError })
    }

    @Test
    fun `reconnect closes previous connection before opening new one`() = runTest {
        val (middleware, factory) = middlewareWithFactory()

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://host:14072"))) { }
        val firstConnection = factory.lastConnection

        middleware.execute(AppState(), AppAction.Connect(ServerEndpoint.Url("ws://other:14072"))) { }

        assertEquals(1, firstConnection?.disconnectCount)
    }

    private fun middlewareWithFactory(sendResult: Boolean = true): Pair<RemoteMiddleware, FakeRemoteConnectionFactory> {
        val factory = FakeRemoteConnectionFactory(sendResult)
        return RemoteMiddleware(connectionFactory = factory) to factory
    }
}

private class FakeRemoteConnection(private val sendResult: Boolean) : RemoteConnection {
    val sentData = mutableListOf<ByteArray>()
    var disconnectCount = 0
        private set

    override fun send(data: ByteArray): Boolean {
        sentData += data
        return sendResult
    }

    override fun disconnect() {
        disconnectCount++
    }
}

private class FakeRemoteConnectionFactory(
    private val sendResult: Boolean = true,
) : RemoteConnectionFactory {
    var lastUrl: String? = null
    var lastListener: RemoteConnectionListener? = null
    var lastConnection: FakeRemoteConnection? = null

    override fun create(url: String, listener: RemoteConnectionListener): RemoteConnection {
        lastUrl = url
        lastListener = listener
        return FakeRemoteConnection(sendResult).also { lastConnection = it }
    }
}
