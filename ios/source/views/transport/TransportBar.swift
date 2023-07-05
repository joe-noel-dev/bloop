import SwiftUI

struct TransportBar: View {
    var playbackState: PlaybackState
    var selections: Selections
    var dispatch: Dispatch

    var body: some View {
        foreground
            .background(.thinMaterial)
    }

    private var foreground: some View {
        HStack {

            loopButton
                .frame(maxWidth: .infinity)

            playButton
                .frame(maxWidth: .infinity)

            queueButton
                .frame(maxWidth: .infinity)

        }
        .frame(maxWidth: .infinity)
    }

    @ViewBuilder
    private var playButton: some View {
        if playbackState.playing != .playing {
            TransportButton(name: "Play", systemImageName: "play.fill") {
                let action = playAction()
                dispatch(action)
            }
        }
        else {
            TransportButton(name: "Stop", systemImageName: "stop.fill") {
                let action = stopAction()
                dispatch(action)
            }
        }

    }

    @ViewBuilder
    private var loopButton: some View {
        TransportButton(
            name: playbackState.looping ? "Exit Loop" : "Enter Loop",
            systemImageName: playbackState.looping ? "repeat.circle.fill" : "repeat.circle"
        ) {
            let action = playbackState.looping ? exitLoopAction() : enterLoopAction()
            dispatch(action)
        }
        .disabled(playbackState.playing != .playing)
        .opacity(playbackState.looping ? 1.0 : 0.5)
        
    }

    @ViewBuilder
    private var emptyButton: some View {
        TransportButton(name: "", systemImageName: "", action: {})
    }

    private enum QueueState {
        case notReady
        case readyToQueue
        case queued
    }

    private var queueState: QueueState {
        if playbackState.playing != .playing {
            return .notReady
        }

        if playbackState.queuedSectionId == selections.section {
            return .queued
        }

        if selections.section != playbackState.sectionId {
            return .readyToQueue
        }

        return .notReady
    }

    @ViewBuilder
    private var queueButton: some View {
        switch queueState {        
        case .readyToQueue, .notReady:
            TransportButton(name: "Jump", systemImageName: "arrow.forward.circle") {
                guard let songId = selections.song, let sectionId = selections.section else {
                    return
                }

                let action = queueAction(song: songId, section: sectionId)
                dispatch(action)
            }
            .disabled(queueState == .notReady)
        case .queued:
            TransportButton(name: "Queued", systemImageName: "checkmark.circle.fill") {}
        }

    }

}

struct TransportButton: View {
    var name: String
    var systemImageName: String
    var action: () -> Void

    var body: some View {
        Button {
            action()
        } label: {
            Label(name, systemImage: systemImageName)
                .labelStyle(.iconOnly)
                .font(.title)
        }
        .frame(width: Layout.touchTarget, height: Layout.touchTarget)
    }
}

struct TransportBar_Previews: PreviewProvider {
    static let playbackState = {
        return PlaybackState.init(playing: .playing)
    }()

    static let selections = Selections()

    static var previews: some View {
        VStack {
            Spacer()
            TransportBar(
                playbackState: playbackState,
                selections: selections,
                dispatch: loggingDispatch
            )
        }
    }
}
