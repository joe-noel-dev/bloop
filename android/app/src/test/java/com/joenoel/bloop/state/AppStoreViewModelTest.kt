package com.joenoel.bloop.state

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class AppStoreViewModelTest {

    private val testDispatcher = StandardTestDispatcher()

    @Before
    fun setUp() {
        Dispatchers.setMain(testDispatcher)
    }

    @After
    fun tearDown() {
        Dispatchers.resetMain()
    }

    @Test
    fun `dispatch updates state via reducer`() {
        val store = AppStoreViewModel()

        store.dispatch(AppAction.SetScanning(true))

        assertEquals(true, store.state.value.scanning)
    }

    @Test
    fun `middleware can dispatch follow up actions`() = runTest(testDispatcher) {
        val middleware = AppMiddleware { _, action, dispatch ->
            if (action is AppAction.ConnectLocal) {
                dispatch(AppAction.SetConnected(ConnectionType.LOCAL))
            }
        }

        val store = AppStoreViewModel(middlewares = listOf(middleware))

        store.dispatch(AppAction.ConnectLocal)
        advanceUntilIdle()

        assertEquals(ConnectionType.LOCAL, store.state.value.connected)
    }
}
