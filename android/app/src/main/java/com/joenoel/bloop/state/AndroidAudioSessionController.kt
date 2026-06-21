package com.joenoel.bloop.state

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager

class AndroidAudioSessionController(context: Context) : AudioSessionController {
    private val audioManager = context.getSystemService(Context.AUDIO_SERVICE) as AudioManager

    override fun requestPlaybackSession(): ReleasableAudioSession? {
        val focusRequest = AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
            .setAudioAttributes(
                AudioAttributes.Builder()
                    .setUsage(AudioAttributes.USAGE_MEDIA)
                    .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
                    .build()
            )
            .setOnAudioFocusChangeListener { }
            .build()

        val granted = audioManager.requestAudioFocus(focusRequest) == AudioManager.AUDIOFOCUS_REQUEST_GRANTED

        if (!granted) {
            return null
        }

        return AndroidPlaybackSession(audioManager, focusRequest)
    }
}

private class AndroidPlaybackSession(
    private val audioManager: AudioManager,
    private val focusRequest: AudioFocusRequest,
) : ReleasableAudioSession {
    private var released = false

    override fun release() {
        if (released) {
            return
        }

        audioManager.abandonAudioFocusRequest(focusRequest)
        released = true
    }
}
