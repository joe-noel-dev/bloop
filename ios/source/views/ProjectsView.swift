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

    private func formatLastSaved(_ rfc3339Timestamp: String) -> String {
        let isoFormatter = ISO8601DateFormatter()
        isoFormatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]

        guard let date = isoFormatter.date(from: rfc3339Timestamp) else {
            return rfc3339Timestamp
        }
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .full
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

struct ProjectsView: View {
    var projects: [Bloop_ProjectInfo]
    var dispatch: Dispatch
    var dismiss: () -> Void

    @State private var selected: String?
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
        }

    }
}

struct ProjectsView_Previews: PreviewProvider {
    static private func projectInfo(name: String, lastSaved: String) -> Bloop_ProjectInfo {
        .with {
            $0.id = name
            $0.name = name
            $0.version = "0"
            $0.lastSaved = lastSaved
        }
    }

    static let projects: [Bloop_ProjectInfo] = [
        projectInfo(name: "Project 1", lastSaved: "2025-05-27T15:34:00Z"),
        projectInfo(name: "Project 2", lastSaved: "2025-05-27T16:34:00Z"),
        projectInfo(name: "Project 3", lastSaved: "2025-05-27T17:34:00Z"),
    ]

    static var previews: some View {
        ProjectsView(projects: projects, dispatch: loggingDispatch) {
            print("Dismiss sheet")
        }
    }
}
