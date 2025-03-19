//
//  AudioSessionMiddleware.swift
//  Bloop
//
//  Created by Joe Noel on 18/03/2025.
//

import AVFoundation

class AudioSessionMiddleware: Middleware {
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {

        if case .connectLocal = action {
            try? AVAudioSession.sharedInstance().setCategory(.playback, mode: .default)
            try? AVAudioSession.sharedInstance().setActive(true)
        }

        if case .disconnect = action {
            try? AVAudioSession.sharedInstance().setActive(false)
        }
    }

}
