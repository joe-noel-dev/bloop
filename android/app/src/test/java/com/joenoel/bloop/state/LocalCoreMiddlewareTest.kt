package com.joenoel.bloop.state

import bloop.Bloop
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class LocalCoreMiddlewareTest {

    private val bloopHome = "${System.getProperty("java.io.tmpdir")}/bloop-test"

    private fun middlewareWith(
        coreFactory: LocalCoreFactory,
        audioController: FakeAudioSessionController = FakeAudioSessionController(),
    ): LocalCoreMiddleware {
        return LocalCoreMiddleware(
            bloopHome = bloopHome,
            audioSessionController = AudioSessionController { audioController.requestPlaybackSession() },
            coreFactory = coreFactory,
        )
    }

    @Test
    fun `connect local starts core and marks local connection`() = runTest {
        val fakeCore = FakeCoreSession()
        val audioController = FakeAudioSessionController()
        var created = 0
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory {
            created += 1
            fakeCore
            },
            audioController = audioController,
        )
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }

        assertEquals(1, created)
        assertEquals(1, audioController.requestCount)
        assertTrue(dispatched.contains(AppAction.SetConnected(ConnectionType.LOCAL)))
        val loadAll = dispatched.filterIsInstance<AppAction.SendRequest>().single()
        assertEquals(Bloop.Entity.ALL, loadAll.request.get.entity)
    }

    @Test
    fun `connect local does not reinitialize existing core`() = runTest {
        val fakeCore = FakeCoreSession()
        val audioController = FakeAudioSessionController()
        var created = 0
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory {
                created += 1
                fakeCore
            },
            audioController = audioController,
        )

        middleware.execute(AppState(), AppAction.ConnectLocal) { }
        middleware.execute(AppState(connected = ConnectionType.LOCAL), AppAction.ConnectLocal) { }

        assertEquals(1, created)
        assertEquals(1, audioController.requestCount)
    }

    @Test
    fun `disconnect while local shuts down core and clears connection`() = runTest {
        val fakeCore = FakeCoreSession()
        val audioController = FakeAudioSessionController()
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory { _ -> fakeCore },
            audioController = audioController,
        )
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }
        middleware.execute(
            AppState(connected = ConnectionType.LOCAL),
            AppAction.Disconnect
        ) { dispatched += it }

        assertEquals(1, fakeCore.closeCount)
        assertEquals(1, audioController.releaseCount)
        assertTrue(dispatched.contains(AppAction.SetConnected(null)))
    }

    @Test
    fun `send raw request forwards to local core`() = runTest {
        val fakeCore = FakeCoreSession(sendResult = true)
        val middleware = middlewareWith(coreFactory = LocalCoreFactory { _ -> fakeCore })
        val payload = byteArrayOf(1, 2, 3)

        middleware.execute(AppState(), AppAction.ConnectLocal) { }
        middleware.execute(
            AppState(connected = ConnectionType.LOCAL),
            AppAction.SendRawRequest(payload)
        ) { }

        assertEquals(1, fakeCore.requests.size)
        assertTrue(payload.contentEquals(fakeCore.requests.first()))
    }

    @Test
    fun `failed send emits error action`() = runTest {
        val fakeCore = FakeCoreSession(sendResult = false)
        val middleware = middlewareWith(coreFactory = LocalCoreFactory { _ -> fakeCore })
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }
        middleware.execute(
            AppState(connected = ConnectionType.LOCAL),
            AppAction.SendRawRequest(byteArrayOf(7))
        ) { dispatched += it }

        assertTrue(dispatched.any { it is AppAction.AddError })
    }

    @Test
    fun `connect remote while local closes embedded core`() = runTest {
        val fakeCore = FakeCoreSession()
        val audioController = FakeAudioSessionController()
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory { _ -> fakeCore },
            audioController = audioController,
        )

        middleware.execute(AppState(), AppAction.ConnectLocal) { }
        middleware.execute(
            AppState(connected = ConnectionType.LOCAL),
            AppAction.Connect(ServerEndpoint.Opaque("remote"))
        ) { }

        assertEquals(1, fakeCore.closeCount)
        assertEquals(1, audioController.releaseCount)
    }

    @Test
    fun `response callback dispatches raw response action`() = runTest {
        var callback: ((ByteArray) -> Unit)? = null
        val fakeCore = FakeCoreSession()
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory { onResponse ->
                callback = onResponse
                fakeCore
            }
        )
        val dispatched = mutableListOf<AppAction>()
        val response = byteArrayOf(9, 8, 7)

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }
        callback?.invoke(response)

        val rawResponses = dispatched.filterIsInstance<AppAction.ReceivedRawResponse>()
        assertEquals(1, rawResponses.size)
        assertTrue(response.contentEquals(rawResponses.first().data))
    }

    @Test
    fun `failed core startup records error and does not connect`() = runTest {
        val audioController = FakeAudioSessionController()
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory { _ -> throw IllegalStateException("boom") },
            audioController = audioController,
        )
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }

        assertTrue(dispatched.any { it is AppAction.AddError })
        assertFalse(dispatched.contains(AppAction.SetConnected(ConnectionType.LOCAL)))
        assertEquals(1, audioController.releaseCount)
    }

    @Test
    fun `failed audio focus records error and does not start local core`() = runTest {
        val audioController = FakeAudioSessionController(allowSession = false)
        var created = 0
        val middleware = middlewareWith(
            coreFactory = LocalCoreFactory {
                created += 1
                FakeCoreSession()
            },
            audioController = audioController,
        )
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }

        assertEquals(0, created)
        assertTrue(dispatched.any { it is AppAction.AddError })
        assertFalse(dispatched.contains(AppAction.SetConnected(ConnectionType.LOCAL)))
        assertEquals(0, audioController.releaseCount)
    }
}

private class FakeCoreSession(
    private val sendResult: Boolean = true,
) : LocalCoreSession {
    val requests = mutableListOf<ByteArray>()
    var closeCount: Int = 0
        private set

    override fun sendRequest(request: ByteArray): Boolean {
        requests += request
        return sendResult
    }

    override fun close() {
        closeCount += 1
    }
}

private class FakeAudioSessionController(
    private val allowSession: Boolean = true,
) {
    var requestCount: Int = 0
        private set
    var releaseCount: Int = 0
        private set

    fun requestPlaybackSession(): ReleasableAudioSession? {
        requestCount += 1
        if (!allowSession) {
            return null
        }

        return object : ReleasableAudioSession {
            private var released = false

            override fun release() {
                if (released) {
                    return
                }

                released = true
                releaseCount += 1
            }
        }
    }
}
