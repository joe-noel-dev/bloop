import SwiftUI

struct TransportBar: View {
    var playbackState: Bloop_PlaybackState
    var project: Bloop_Project
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
        .padding([.top, .bottom])
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
            systemImageName: "repeat"
        ) {
            let action = playbackState.looping ? exitLoopAction() : enterLoopAction()
            dispatch(action)
        }
        .disabled(playbackState.playing != .playing)
        .foregroundColor(playbackState.looping ? .accentColor : .primary)
        .opacity(playbackState.playing == .playing ? 1.0 : 0.5)

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

        if playbackState.queuedSectionID == project.selections.section {
            return .queued
        }

        if project.selections.section != playbackState.sectionID {
            return .readyToQueue
        }

        return .notReady
    }

    @ViewBuilder
    private var queueButton: some View {
        switch queueState {
        case .readyToQueue, .notReady:
            TransportButton(name: "Jump", systemImageName: "arrow.right") {
                let songId = project.selections.song
                let sectionId = project.selections.section

                guard songId != 0, sectionId != 0 else {
                    return
                }

                let action = queueAction(song: songId, section: sectionId)
                dispatch(action)
            }
            .foregroundColor(.accentColor)
            .disabled(queueState == .notReady)
        case .queued:
            TransportButton(name: "Queued", systemImageName: "checkmark") {}.foregroundColor(
                .accentColor
            )
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
                .font(.system(size: 36))
        }
    }
}

struct TransportBar_Previews: PreviewProvider {
    static let playbackState = {
        Bloop_PlaybackState.with {
            $0.playing = .playing
        }
    }()

    static let project = demoProject()

    static var previews: some View {
        VStack {
            Spacer()
            TransportBar(
                playbackState: playbackState,
                project: project,
                dispatch: loggingDispatch
            )
        }
    }
}
