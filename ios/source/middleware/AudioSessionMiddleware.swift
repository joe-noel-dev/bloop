//
//  AudioSessionMiddleware.swift
//  Bloop
//
//  Created by Joe Noel on 18/03/2025.
//

import AVFoundation

enum AudioSessionConfigurator {
    static func activate(preferences: Bloop_AudioPreferences? = nil) {
        let session = AVAudioSession.sharedInstance()

        do {
            try session.setCategory(.playback, mode: .default)

            if let preferences, preferences.sampleRate > 0 {
                try session.setPreferredSampleRate(Double(preferences.sampleRate))

                if preferences.bufferSize > 0 {
                    try session.setPreferredIOBufferDuration(Double(preferences.bufferSize) / Double(preferences.sampleRate))
                }
            }

            try session.setActive(true)
        } catch {
            print("Unable to configure audio session: \(error)")
        }
    }
}

class AudioSessionMiddleware: Middleware {
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {

        if case .connectLocal = action {
            AudioSessionConfigurator.activate(preferences: state.preferences?.audio)
        }

        if case .setPreferences(let preferences) = action {
            AudioSessionConfigurator.activate(preferences: preferences.audio)
        }

        if case .sendRequest(let request) = action,
           request.hasUpdate,
           request.update.hasPreferences {
            AudioSessionConfigurator.activate(preferences: request.update.preferences.audio)
        }

        if case .disconnect = action {
            try? AVAudioSession.sharedInstance().setActive(false)
        }
    }

}
