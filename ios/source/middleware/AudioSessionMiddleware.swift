//
//  AudioSessionMiddleware.swift
//  Bloop
//
//  Created by Joe Noel on 18/03/2025.
//

import AVFoundation

enum AudioSessionConfigurator {
    private static let defaultSampleRate: UInt32 = 48_000
    private static let defaultBufferSize: UInt32 = 512
    private static let defaultMainChannelOffset: UInt32 = 0
    private static let defaultClickChannelOffset: UInt32 = 2

    static func activate(preferences: Bloop_AudioPreferences? = nil) {
        let session = AVAudioSession.sharedInstance()
        let audioPreferences = resolvedPreferences(preferences)

        do {
            try session.setCategory(.playback, mode: .default)
            try session.setPreferredSampleRate(Double(audioPreferences.sampleRate))
            try session.setPreferredIOBufferDuration(
                Double(audioPreferences.bufferSize) / Double(audioPreferences.sampleRate)
            )

            configureOutputChannelCount(session: session, preferences: audioPreferences)
            try session.setActive(true)
        } catch {
            print("Unable to configure audio session: \(error)")
        }
    }

    private static func resolvedPreferences(_ preferences: Bloop_AudioPreferences?) -> Bloop_AudioPreferences {
        guard let preferences else {
            return defaultPreferences()
        }

        var resolved = preferences

        if resolved.sampleRate <= 0 {
            resolved.sampleRate = defaultSampleRate
        }

        if resolved.bufferSize <= 0 {
            resolved.bufferSize = defaultBufferSize
        }

        return resolved
    }

    private static func defaultPreferences() -> Bloop_AudioPreferences {
        .with {
            $0.sampleRate = defaultSampleRate
            $0.bufferSize = defaultBufferSize
            $0.mainChannelOffset = defaultMainChannelOffset
            $0.clickChannelOffset = defaultClickChannelOffset
        }
    }

    private static func configureOutputChannelCount(
        session: AVAudioSession,
        preferences: Bloop_AudioPreferences
    ) {
        let requiredChannelCount = max(
            Int(preferences.mainChannelOffset) + 2,
            Int(preferences.clickChannelOffset) + 2
        )

        guard requiredChannelCount > 0 else {
            return
        }

        let maximumChannelCount = session.maximumOutputNumberOfChannels
        let preferredChannelCount = maximumChannelCount > 0
            ? min(requiredChannelCount, maximumChannelCount)
            : requiredChannelCount

        guard session.outputNumberOfChannels != preferredChannelCount else {
            return
        }

        do {
            try session.setPreferredOutputNumberOfChannels(preferredChannelCount)
        } catch {
            print("Unable to configure audio session channel count: \(error)")
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
