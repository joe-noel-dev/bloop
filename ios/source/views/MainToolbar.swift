import SwiftUI

enum EditingEntity {
    case projects
    case projectName
    case songs
}

enum ToolbarAction {
    case disconnect
    case connectToServer(Server)
    case connectLocal
}

struct MainToolbar: ToolbarContent {
    var currentSong: Bloop_Song
    var servers: [Server]
    var scanning: Bool

    @Binding var editingEntity: EditingEntity?
    @Environment(\.editMode) var editMode
    var onAction: (ToolbarAction) -> Void
    
    @State private var showingServerSelection = false

    var body: some ToolbarContent {

        ToolbarItemGroup(placement: .navigationBarLeading) {
            Button {
                editingEntity = .projects
            } label: {
                Image(systemName: "folder")
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
                showingServerSelection = true
            } label: {
                Image(systemName: "network")
            }
            .sheet(isPresented: $showingServerSelection) {
                ServerSelectionView(
                    servers: servers,
                    scanning: scanning,
                    onServerSelected: { server in
                        showingServerSelection = false
                        onAction(.connectToServer(server))
                    },
                    onLocalSelected: {
                        showingServerSelection = false
                        onAction(.connectLocal)
                    },
                    onCancel: {
                        showingServerSelection = false
                    }
                )
            }
        }
    }
}
