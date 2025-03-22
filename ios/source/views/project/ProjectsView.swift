import SwiftUI
import UniformTypeIdentifiers

struct ProjectPreview: View {
    var project: Bloop_ProjectInfo
    var selected: Bool

    var body: some View {
        HStack(spacing: Layout.units(2)) {
            Text(project.name)
                .font(.headline)

            if selected {
                Spacer()
                Text("Last saved \(formatLastSaved(project.lastSaved))")
                    .font(.subheadline)
            }
        }
    }

    private func formatLastSaved(_ millisecondsSince1970: Int64) -> String {
        let secondsSince1970 = millisecondsSince1970 / 1000
        let interval = TimeInterval(secondsSince1970)
        let date = Date(timeIntervalSince1970: interval)
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .full
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

struct ProjectsView: View {
    var projects: [Bloop_ProjectInfo]
    var dispatch: Dispatch
    var dismiss: () -> Void

    @State private var selected: Bloop_ProjectInfo.ID?
    @State private var showImportFileDialog: Bool = false
    @State private var selectedFileURL: URL?

    private var sortedProjects: [Bloop_ProjectInfo] {
        projects.sorted { a, b in
            a.lastSaved > b.lastSaved
        }
    }

    var body: some View {

        NavigationStack {

            List(selection: $selected) {
                ForEach(sortedProjects) { project in
                    ProjectPreview(project: project, selected: selected == project.id)
                }
                .onDelete { offsets in
                    let projectIds = offsets.map { offset in
                        sortedProjects[offset].id
                    }

                    for projectId in projectIds {
                        let action = removeProjectAction(projectId)
                        dispatch(action)
                    }

                }
            }
            .listStyle(.plain)
            .navigationTitle("Projects")
            .toolbar {

                if let selected = selected {
                    Button {
                        let action = loadProjectAction(selected)
                        dispatch(action)
                        dismiss()
                    } label: {
                        Label("Open", systemImage: "folder")
                            .labelStyle(.titleOnly)
                    }

                    Button {
                        let action = duplicateProjectAction(selected)
                        dispatch(action)
                        dismiss()
                    } label: {
                        Label("Duplicate", systemImage: "doc.on.doc")
                            .labelStyle(.titleOnly)
                    }
                }

                Button {
                    showImportFileDialog = true
                } label: {
                    Label("Import", systemImage: "square.and.arrow.down")
                }

                Button {
                    let action = newProjectAction()
                    dispatch(action)
                    dismiss()
                } label: {
                    Label("New", systemImage: "plus")
                }

            }
            .padding(Layout.units(2))
            .onAppear {
                let action = getProjectsAction()
                dispatch(action)
            }
            .fileImporter(isPresented: $showImportFileDialog, allowedContentTypes: [UTType.data]) {
                result in
                switch result {
                case .success(let url):
                    print("Selected file for import: \(url)")
                    dispatch(.importProject(url))

                case .failure(let error):
                    print("Error selecting file: \(error.localizedDescription)")
                }
            }
        }

    }
}

struct ProjectsView_Previews: PreviewProvider {
    static private func projectInfo(name: String, savedAgo: Int) -> Bloop_ProjectInfo {
        .with {
            $0.id = randomId()
            $0.name = name
            $0.version = "0"
            $0.lastSaved = Int64((Date() - TimeInterval(savedAgo)).timeIntervalSince1970.magnitude) * 1000
        }
    }
    
    static let projects: [Bloop_ProjectInfo] = [
        projectInfo(name: "Project 1", savedAgo: 19),
        projectInfo(name: "Project 2", savedAgo: 20),
        projectInfo(name: "Project 3", savedAgo: 32478),
    ]

    static var previews: some View {
        ProjectsView(projects: projects, dispatch: loggingDispatch) {
            print("Dismiss sheet")
        }
    }
}
