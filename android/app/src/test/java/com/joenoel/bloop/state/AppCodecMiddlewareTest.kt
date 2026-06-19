package com.joenoel.bloop.state

import bloop.Bloop
import bloop.playbackState
import bloop.request
import bloop.response
import bloop.transportRequest
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AppCodecMiddlewareTest {

    @Test
    fun `send request emits raw request bytes`() = runTest {
        val middleware = AppCodecMiddleware()
        val dispatched = mutableListOf<AppAction>()
        val original = request {
            transport = transportRequest {
                method = Bloop.TransportMethod.PLAY
            }
        }

        middleware.execute(AppState(), AppAction.SendRequest(original)) { dispatched += it }

        val rawRequest = dispatched.single { it is AppAction.SendRawRequest } as AppAction.SendRawRequest
        assertTrue(original.toByteArray().contentEquals(rawRequest.data))
    }

    @Test
    fun `received raw response emits typed response action`() = runTest {
        val middleware = AppCodecMiddleware()
        val dispatched = mutableListOf<AppAction>()
        val original = response {
            playbackState = playbackState {
                playing = Bloop.PlayingState.PLAYING
                songId = 42L
            }
        }

        middleware.execute(AppState(), AppAction.ReceivedRawResponse(original.toByteArray())) { dispatched += it }

        val decodedResponse = dispatched.single { it is AppAction.ReceivedResponse } as AppAction.ReceivedResponse
        assertEquals(Bloop.PlayingState.PLAYING, decodedResponse.response.playbackState.playing)
        assertEquals(42L, decodedResponse.response.playbackState.songId)
    }
}