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
        HStack(alignment: .center) {

            Spacer()

            loopButton
            
            Spacer()

            if playbackState.playing == .playing {
                stopButton
            }
            else {
                playButton
            }
            
            Spacer()

            queueButton

            Spacer()

        }
    }

    @ViewBuilder
    private var playButton: some View {
        TransportButton(systemImageName: "play.fill") {
            let action = playAction()
            dispatch(action)
        }
    }

    @ViewBuilder
    private var stopButton: some View {
        TransportButton(systemImageName: "stop.fill") {
            let action = stopAction()
            dispatch(action)
        }
    }

    @ViewBuilder
    private var loopButton: some View {
        if playbackState.playing != .playing {
            emptyButton
        }
        else {
            TransportButton(systemImageName: "repeat") {
                let action = playbackState.looping ? exitLoopAction() : enterLoopAction()
                dispatch(action)
            }
            .opacity(playbackState.looping ? 1.0 : 0.5)
        }
    }

    @ViewBuilder
    private var emptyButton: some View {
        Spacer().frame(width: Layout.touchTarget, height: Layout.touchTarget)
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
        case .notReady:
            emptyButton
        case .readyToQueue:
            TransportButton(systemImageName: "arrow.forward") {
                guard let songId = selections.song, let sectionId = selections.section else {
                    return
                }

                let action = queueAction(song: songId, section: sectionId)
                dispatch(action)
            }
        case .queued:
            TransportButton(systemImageName: "checkmark") {}
        }

    }

}

struct TransportButton: View {
    var systemImageName: String
    var action: () -> Void

    var body: some View {
        Button {
            action()
        } label: {
            Image(systemName: systemImageName)
                .resizable()
                .padding(Layout.units(1.5))
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
