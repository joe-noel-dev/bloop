package com.joenoel.bloop.state

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class DiscoveryMiddlewareTest {

    @Test
    fun `restart scan clears servers and starts discovery`() = runTest {
        val controller = FakeServiceDiscoveryController()
        val middleware = DiscoveryMiddleware(controller)
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.RestartScan) { dispatched += it }

        assertTrue(dispatched.first() is AppAction.RemoveAllServers)
        assertTrue(controller.wasRestarted)
    }

    @Test
    fun `controller callbacks dispatch scanning servers and errors`() = runTest {
        val controller = FakeServiceDiscoveryController()
        val middleware = DiscoveryMiddleware(controller)
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.RestartScan) { dispatched += it }

        val discovered = listOf(ServerEndpoint.HostPort("192.168.1.15", 14072))
        controller.emitScanning(true)
        controller.emitServers(discovered)
        controller.emitError("boom")

        assertTrue(dispatched.contains(AppAction.SetScanning(true)))
        assertTrue(dispatched.contains(AppAction.SetDiscoveredServers(discovered)))
        assertTrue(dispatched.contains(AppAction.AddError("boom")))
    }

    @Test
    fun `non restart actions are ignored`() = runTest {
        val controller = FakeServiceDiscoveryController()
        val middleware = DiscoveryMiddleware(controller)
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.ConnectLocal) { dispatched += it }

        assertEquals(0, dispatched.size)
        assertTrue(!controller.wasRestarted)
    }

    @Test
    fun `stop scan calls controller stop`() = runTest {
        val controller = FakeServiceDiscoveryController()
        val middleware = DiscoveryMiddleware(controller)
        val dispatched = mutableListOf<AppAction>()

        middleware.execute(AppState(), AppAction.StopScan) { dispatched += it }

        assertEquals(0, dispatched.size)
        assertTrue(controller.wasStopped)
    }
}

private class FakeServiceDiscoveryController : ServiceDiscoveryController {
    var wasRestarted: Boolean = false
        private set
    var wasStopped: Boolean = false
        private set

    private var onScanningChanged: ((Boolean) -> Unit)? = null
    private var onServersChanged: ((List<ServerEndpoint>) -> Unit)? = null
    private var onError: ((String) -> Unit)? = null

    override fun restart(
        onScanningChanged: (Boolean) -> Unit,
        onServersChanged: (List<ServerEndpoint>) -> Unit,
        onError: (String) -> Unit,
    ) {
        wasRestarted = true
        this.onScanningChanged = onScanningChanged
        this.onServersChanged = onServersChanged
        this.onError = onError
    }

    override fun stop() {
        wasStopped = true
    }

    fun emitScanning(scanning: Boolean) {
        onScanningChanged?.invoke(scanning)
    }

    fun emitServers(servers: List<ServerEndpoint>) {
        onServersChanged?.invoke(servers)
    }

    fun emitError(message: String) {
        onError?.invoke(message)
    }
}
