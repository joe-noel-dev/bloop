package com.joenoel.bloop

import bloop.Bloop
import bloop.playbackState
import bloop.request
import bloop.response
import bloop.transportRequest
import org.junit.Assert.assertEquals
import org.junit.Test

class ProtoCodecTest {

    @Test
    fun `transport request round-trips through protobuf encoding`() {
        val original = request {
            transport = transportRequest {
                method = Bloop.TransportMethod.PLAY
            }
        }

        val bytes = original.toByteArray()
        val decoded = Bloop.Request.parseFrom(bytes)

        assertEquals(Bloop.TransportMethod.PLAY, decoded.transport.method)
    }

    @Test
    fun `response with playback state round-trips through protobuf encoding`() {
        val original = response {
            playbackState = playbackState {
                playing = Bloop.PlayingState.PLAYING
                songId = 42L
                sectionId = 7L
                looping = true
            }
        }

        val bytes = original.toByteArray()
        val decoded = Bloop.Response.parseFrom(bytes)

        assertEquals(Bloop.PlayingState.PLAYING, decoded.playbackState.playing)
        assertEquals(42L, decoded.playbackState.songId)
        assertEquals(7L, decoded.playbackState.sectionId)
        assertEquals(true, decoded.playbackState.looping)
    }

    @Test
    fun `empty request encodes and decodes without error`() {
        val bytes = Bloop.Request.getDefaultInstance().toByteArray()
        val decoded = Bloop.Request.parseFrom(bytes)
        assertEquals(Bloop.Request.getDefaultInstance(), decoded)
    }
}
