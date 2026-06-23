package com.joenoel.bloop.state

import bloop.Bloop
import bloop.audioPreferences
import bloop.midiDevices
import bloop.playbackState
import bloop.preferences
import bloop.progress
import bloop.project
import bloop.projectInfo
import bloop.projectSyncResponse
import bloop.response
import bloop.uploadAck
import bloop.user
import bloop.userStatusResponse
import bloop.waveformData
import bloop.waveformResponse
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ResponseMiddlewareTest {

    @Test
    fun `received response fans out typed state actions`() = runTest {
        val middleware = ResponseMiddleware()
        val dispatched = mutableListOf<AppAction>()
        val response = response {
            project = project { }
            playbackState = playbackState {
                playing = Bloop.PlayingState.PLAYING
                songId = 99L
            }
            progress = progress {
                songProgress = 1.0
            }
            projects += projectInfo {
                id = "project-1"
                name = "Local"
            }
            cloudProjects += projectInfo {
                id = "project-2"
                name = "Cloud"
            }
            error = "boom"
            waveform = waveformResponse {
                sampleId = 77L
                waveformData = waveformData {
                    sampleRate = 44100
                }
            }
            upload = uploadAck {
                uploadId = 5L
            }
            projectInfo = projectInfo {
                id = "project-3"
                name = "Info"
            }
            userStatus = userStatusResponse {
                user = user {
                    id = "user-1"
                    email = "test@example.com"
                    name = "Test User"
                }
                state = Bloop.UserState.USER_STATE_SIGNED_IN
            }
            projectSync = projectSyncResponse {
                projectId = "project-4"
                status = Bloop.SyncStatus.SYNC_STATUS_COMPLETE
            }
            preferences = preferences {
                audio = audioPreferences {
                    outputDevice = "built-in"
                }
            }
        }

        middleware.execute(AppState(), AppAction.ReceivedResponse(response)) { dispatched += it }

        assertTrue(dispatched.contains(AppAction.SetProject(response.project)))
        assertTrue(dispatched.contains(AppAction.SetPlaybackState(response.playbackState)))
        assertTrue(dispatched.contains(AppAction.SetProgress(response.progress)))
        assertTrue(dispatched.contains(AppAction.SetProjects(response.projectsList)))
        assertTrue(dispatched.contains(AppAction.SetCloudProjects(response.cloudProjectsList)))
        assertTrue(dispatched.contains(AppAction.AddError("boom")))
        assertTrue(dispatched.contains(AppAction.AddWaveform(77L, response.waveform.waveformData)))
        assertTrue(dispatched.contains(AppAction.UploadAck(5L)))
        assertTrue(dispatched.contains(AppAction.SetProjectInfo(response.projectInfo)))
        assertTrue(dispatched.contains(AppAction.SetUser(response.userStatus.user)))
        assertTrue(dispatched.contains(AppAction.SetProjectSync("project-4", Bloop.SyncStatus.SYNC_STATUS_COMPLETE)))
        assertTrue(dispatched.contains(AppAction.SetPreferences(response.preferences)))
        assertEquals(13, dispatched.size)
    }

    @Test
    fun `received response with empty user status clears user`() = runTest {
        val middleware = ResponseMiddleware()
        val dispatched = mutableListOf<AppAction>()
        val response = response {
            userStatus = userStatusResponse {
                state = Bloop.UserState.USER_STATE_SIGNED_OUT
            }
        }

        middleware.execute(AppState(), AppAction.ReceivedResponse(response)) { dispatched += it }

        assertEquals(2, dispatched.size)
        assertEquals(AppAction.SetLastResponseText(response.toString()), dispatched.first())
        assertEquals(AppAction.ClearUser, dispatched.last())
    }

    @Test
    fun `received response with midi devices dispatches set midi devices`() = runTest {
        val middleware = ResponseMiddleware()
        val dispatched = mutableListOf<AppAction>()
        val response = response {
            midiDevices = midiDevices {
                portNames += "iCON G_Boar V1.03"
                portNames += "USB MIDI Interface"
            }
        }

        middleware.execute(AppState(), AppAction.ReceivedResponse(response)) { dispatched += it }

        assertTrue(dispatched.contains(AppAction.SetMidiDevices(response.midiDevices)))
    }
}