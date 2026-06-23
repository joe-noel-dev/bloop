package com.joenoel.bloop.state

import bloop.Bloop
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class AppReducerTest {

    @Test
    fun `set project replaces project in state`() {
        val state = AppState()
        val nextProject = Bloop.Project
            .newBuilder()
            .setSelections(
                Bloop.Selections
                    .newBuilder()
                    .setSong(11L)
                    .setSection(7L)
                    .build()
            )
            .build()

        val nextState = AppReducer.reduce(state, AppAction.SetProject(nextProject))

        assertEquals(11L, nextState.project.selections.song)
        assertEquals(7L, nextState.project.selections.section)
    }

    @Test
    fun `set and dismiss project sync updates map`() {
        val state = AppState()

        val withSync = AppReducer.reduce(
            state,
            AppAction.SetProjectSync("project-1", Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS)
        )

        assertEquals(1, withSync.projectSyncStatuses.size)
        assertEquals(
            Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS,
            withSync.projectSyncStatuses["project-1"]
        )

        val dismissed = AppReducer.reduce(withSync, AppAction.DismissProjectSync("project-1"))

        assertTrue(dismissed.projectSyncStatuses.isEmpty())
    }

    @Test
    fun `error list appends and clear user resets user`() {
        val user = Bloop.User.newBuilder().setId("u1").setEmail("u1@example.com").build()
        val withUser = AppReducer.reduce(AppState(), AppAction.SetUser(user))
        assertEquals("u1", withUser.user?.id)

        val withError = AppReducer.reduce(withUser, AppAction.AddError("boom"))
        assertEquals(listOf("boom"), withError.errors)

        val noUser = AppReducer.reduce(withError, AppAction.ClearUser)
        assertNull(noUser.user)
    }

    @Test
    fun `waveform add and remove updates cache`() {
        val waveform = Bloop.WaveformData
            .newBuilder()
            .setSampleRate(44100)
            .build()

        val withWaveform = AppReducer.reduce(AppState(), AppAction.AddWaveform(55L, waveform))
        assertTrue(withWaveform.waveforms.containsKey(55L))

        val withoutWaveform = AppReducer.reduce(withWaveform, AppAction.RemoveWaveform(55L))
        assertFalse(withoutWaveform.waveforms.containsKey(55L))
    }

    @Test
    fun `set midi devices stores devices in state`() {
        val midiDevices = Bloop.MidiDevices
            .newBuilder()
            .addPortNames("iCON G_Boar V1.03")
            .addPortNames("USB MIDI Interface")
            .build()

        val nextState = AppReducer.reduce(AppState(), AppAction.SetMidiDevices(midiDevices))

        assertEquals(2, nextState.midiDevices?.portNamesList?.size)
        assertEquals("iCON G_Boar V1.03", nextState.midiDevices?.portNamesList?.first())
    }
}
