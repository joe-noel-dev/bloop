import SwiftUI

struct TransportBar: View {
    var state: AppState
    var dispatch: Dispatch

    #if os(iOS)
        @Environment(\.horizontalSizeClass) var horizontalSizeClass
    #endif

    var body: some View {
        VStack(spacing: Layout.units(2)) {

            MetronomeView(
                isPlaying: playbackState.playing == .playing,
                sectionBeat: progress.sectionBeat
            )

            beatPositionView

            HStack(alignment: .center, spacing: Layout.units(4)) {
                backButton
                    .frame(width: Layout.units(4))
                Spacer()
                loopButton
                    .frame(width: Layout.units(4))
                playButton
                    .frame(width: Layout.units(4))
                queueButton
                    .frame(width: Layout.units(4))
                Spacer()
                forwardButton
                    .frame(width: Layout.units(4))
            }
            .padding(Layout.units(2))
            .frame(maxWidth: .infinity)
            .background(.thinMaterial)
        }
    }

    private var playbackState: Bloop_PlaybackState {
        state.playbackState
    }

    private var project: Bloop_Project {
        state.project
    }

    private var progress: Bloop_Progress {
        state.progress
    }

    private var playingSong: Bloop_Song? {
        project.songs.first { $0.id == playbackState.songID }
    }

    private var playingSection: Bloop_Section? {
        playingSong?.sections.first { $0.id == playbackState.sectionID }
    }

    private var sectionBeatDisplay: Int {
        Int(progress.sectionBeat)
    }

    private var songBeatDisplay: Int {
        let sectionStart = playingSection?.start ?? 0
        return Int(progress.sectionBeat + sectionStart)
    }

    private var isCompact: Bool {
        #if os(iOS)
            return horizontalSizeClass == .compact
        #else
            return false
        #endif
    }

    @ViewBuilder
    private var beatPositionView: some View {
        if playbackState.playing == .playing {
            HStack(spacing: Layout.units(3)) {
                BeatCounter(label: isCompact ? nil : "Sec", value: sectionBeatDisplay)
                BeatCounter(label: isCompact ? nil : "Song", value: songBeatDisplay, color: .secondary)
            }
        }
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

    @ViewBuilder
    private var backButton: some View {
        TransportButton(name: "Previous Song", systemImageName: "chevron.backward.to.line") {
            selectPreviousSong()
        }
        .disabled(previousSongId(project) == nil)
    }

    @ViewBuilder
    private var forwardButton: some View {
        TransportButton(name: "Next Song", systemImageName: "chevron.forward.to.line") {
            selectNextSong()
        }
        .disabled(nextSongId(project) == nil)
    }

    private func selectPreviousSong() {
        selectSongWithOffset(-1)
    }

    private func selectNextSong() {
        selectSongWithOffset(1)
    }

    private func selectSongWithOffset(_ offset: Int) {
        let selectedSongId = project.selections.song
        guard let index = project.songs.firstIndex(where: { $0.id == selectedSongId }) else {
            return
        }

        let nextIndex = index + offset

        guard 0 <= nextIndex && nextIndex < state.project.songs.count else {
            return
        }

        let nextSong = state.project.songs[nextIndex].id
        let action = selectSongAction(nextSong)
        dispatch(action)
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

struct BeatCounter: View {
    var label: String?
    var value: Int
    var color: Color = .primary

    var body: some View {
        HStack(spacing: Layout.units(0.5)) {
            if let label {
                Text(label)
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            Text(String(value))
                .font(.system(.body, design: .monospaced))
                .fontWeight(.bold)
                .foregroundColor(color)
                .frame(minWidth: Layout.units(5), alignment: .leading)
        }
    }
}

private let state = {
    var appState = AppState()
    appState.project = demoProject()
    appState.progress = Bloop_Progress()
    return appState
}()

struct TransportBar_Previews: PreviewProvider {

    static var previews: some View {
        VStack {
            Spacer()
            TransportBar(
                state: state,
                dispatch: loggingDispatch
            )
        }
    }
}
