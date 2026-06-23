package com.joenoel.bloop.state

import bloop.Bloop

class ResponseMiddleware : AppMiddleware {
    override suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit) {
        if (action is AppAction.ReceivedResponse) {
            onResponse(action.response, dispatch)
        }
    }

    private fun onResponse(response: Bloop.Response, dispatch: (AppAction) -> Unit) {
        dispatch(AppAction.SetLastResponseText(response.toString()))

        if (response.hasProject()) {
            dispatch(AppAction.SetProject(response.project))
        }

        if (response.hasPlaybackState()) {
            dispatch(AppAction.SetPlaybackState(response.playbackState))
        }

        if (response.hasProgress()) {
            dispatch(AppAction.SetProgress(response.progress))
        }

        if (response.projectsList.isNotEmpty()) {
            dispatch(AppAction.SetProjects(response.projectsList))
        }

        if (response.cloudProjectsList.isNotEmpty()) {
            dispatch(AppAction.SetCloudProjects(response.cloudProjectsList))
        }

        if (response.error.isNotEmpty()) {
            dispatch(AppAction.AddError(response.error))
        }

        if (response.hasWaveform()) {
            val waveform = response.waveform
            dispatch(AppAction.AddWaveform(waveform.sampleId, waveform.waveformData))
        }

        if (response.hasUpload()) {
            dispatch(AppAction.UploadAck(response.upload.uploadId))
        }

        if (response.hasProjectInfo()) {
            dispatch(AppAction.SetProjectInfo(response.projectInfo))
        }

        if (response.hasUserStatus()) {
            if (response.userStatus.hasUser()) {
                dispatch(AppAction.SetUser(response.userStatus.user))
            } else {
                dispatch(AppAction.ClearUser)
            }
        }

        if (response.hasProjectSync()) {
            dispatch(AppAction.SetProjectSync(response.projectSync.projectId, response.projectSync.status))
        }

        if (response.hasPreferences()) {
            dispatch(AppAction.SetPreferences(response.preferences))
        }

        if (response.hasAudioDevices()) {
            dispatch(AppAction.SetAudioDevices(response.audioDevices))
        }

        if (response.hasAudioStatus()) {
            dispatch(AppAction.SetAudioStatus(response.audioStatus))
        }

        if (response.hasMidiDevices()) {
            dispatch(AppAction.SetMidiDevices(response.midiDevices))
        }
    }
}