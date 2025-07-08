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
        
        ToolbarItem(placement: .navigationBarLeading) {
            Button("Songs", systemImage: "music.note.list") {
                editingEntity = .songs
            }
        }

        ToolbarItemGroup(placement: .navigationBarTrailing) {
            Menu {
                Button("Projects", systemImage: "folder") {
                    editingEntity = .projects
                }
                
                if editMode?.wrappedValue == .active {
                    Button("Rename Project", systemImage: "pencil") {
                        editingEntity = .projectName
                    }
                }
                
                Divider()
                
                Button("Connect to Server", systemImage: "network") {
                    showingServerSelection = true
                }
                
                Button("Connect Local", systemImage: "desktopcomputer") {
                    onAction(.connectLocal)
                }
                
                Divider()
                
                Button("Disconnect", systemImage: "xmark.circle", role: .destructive) {
                    onAction(.disconnect)
                }
            } label: {
                Image(systemName: "ellipsis.circle")
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
