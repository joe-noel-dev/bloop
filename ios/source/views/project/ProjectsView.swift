import SwiftUI

struct ProjectPreview: View {
    var project: ProjectInfo
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
    var projects: [ProjectInfo]
    var dispatch: Dispatch
    var dismiss: () -> Void

    @State private var selected: ProjectInfo.ID?

    private var sortedProjects: [ProjectInfo] {
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
                    offsets.map { offset in
                        sortedProjects[offset].id
                    }.forEach { projectId in
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
    static let projects: [ProjectInfo] = [
        ProjectInfo(
            id: "id-1",
            name: "Project 1",
            version: "0",
            lastSaved: Int64((Date() - 19).timeIntervalSince1970.magnitude) * 1000
        ),
        ProjectInfo(
            id: "id-2",
            name: "Project 2",
            version: "0",
            lastSaved: Int64((Date() - 20).timeIntervalSince1970.magnitude) * 1000
        ),
        ProjectInfo(
            id: "id-3",
            name: "Project 3",
            version: "0",
            lastSaved: Int64((Date() - 32478).timeIntervalSince1970.magnitude) * 1000
        ),
        ProjectInfo(
            id: "id-4",
            name: "Project 4",
            version: "0",
            lastSaved: Int64((Date() - 37_647_823).timeIntervalSince1970.magnitude) * 1000
        ),
        ProjectInfo(
            id: "id-5",
            name: "Project 5",
            version: "0",
            lastSaved: Int64((Date() - 327863).timeIntervalSince1970.magnitude) * 1000
        ),
        ProjectInfo(
            id: "id-6",
            name: "Project 6",
            version: "0",
            lastSaved: Int64((Date() - 876870).timeIntervalSince1970.magnitude) * 1000
        ),

    ]

    static var previews: some View {
        ProjectsView(projects: projects, dispatch: loggingDispatch) {
            print("Dismiss sheet")
        }
    }
}
