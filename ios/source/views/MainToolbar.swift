import SwiftUI

enum EditingEntity {
    case projects
    case projectName
    case songs
}

enum ToolbarAction {
    case disconnect
}

struct MainToolbar: ToolbarContent {
    var currentSong: Bloop_Song

    @Binding var editingEntity: EditingEntity?
    @Environment(\.editMode) var editMode
    var onAction: (ToolbarAction) -> Void

    var body: some ToolbarContent {

        ToolbarItemGroup(placement: .navigationBarLeading) {
            Button {
                editingEntity = .projects
            } label: {
                Image(systemName: "externaldrive")
            }

            Button {
                editingEntity = .songs
            } label: {
                Image(systemName: "music.note.list")
            }
        }

        ToolbarItemGroup(placement: .navigationBarTrailing) {
            if editMode?.wrappedValue == .active {
                Menu {
                    Button("Rename Project", systemImage: "pencil") {
                        editingEntity = .projectName
                    }

                } label: {
                    Image(systemName: "ellipsis.circle")
                }
            }

            Button(role: .destructive) {
                onAction(.disconnect)
            } label: {
                Image(systemName: "xmark")
            }
        }
    }
}
