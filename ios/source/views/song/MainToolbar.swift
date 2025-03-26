import SwiftUI

struct MainToolbar: View {

    var currentSong: Bloop_Song

    @Binding var editingSections: Bool
    @Binding var editingSample: Bool
    @Binding var editingProjects: Bool
    @Binding var editingProjectName: Bool

    var dispatch: Dispatch

    var body: some View {

        Menu {
            projectsButton
            renameProjectButton

            sectionsButton
            addSampleButton

            removeButton
            disconnectButton
        } label: {
            Image(systemName: "ellipsis")
        }
        .font(.title)

    }

    @ViewBuilder
    private var sectionsButton: some View {
        Button {
            editingSections = true
        } label: {
            Label("Sections", systemImage: "rectangle.grid.1x2")
        }
    }

    @ViewBuilder
    private var addSampleButton: some View {
        Button {
            editingSample = true
        } label: {
            Label(
                !currentSong.hasSample ? "Add Sample" : "Replace Sample",
                systemImage: "waveform"
            )
        }
    }

    @ViewBuilder
    private var removeButton: some View {
        Button(role: .destructive) {
            let action = removeSongAction(currentSong.id)
            dispatch(action)
        } label: {
            Label("Remove Song", systemImage: "trash")
        }
    }

    @ViewBuilder
    private var projectsButton: some View {
        Button {
            editingProjects = true
        } label: {
            Label("Projects", systemImage: "externaldrive")
        }
    }

    @ViewBuilder
    private var renameProjectButton: some View {
        Button {
            editingProjectName = true
        } label: {
            Label("Rename Project", systemImage: "pencil")
        }
    }

    @ViewBuilder
    private var disconnectButton: some View {
        Button(role: .destructive) {
            dispatch(.disconnect)
        } label: {
            Label("Disconnect", systemImage: "phone.down.fill")
        }
    }
}
