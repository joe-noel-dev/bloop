import SwiftUI
import UniformTypeIdentifiers

struct ProjectSyncNotificationView: View {
    let projectId: String
    let syncStatus: Bloop_SyncStatus
    let projects: [Bloop_ProjectInfo]
    let onDismiss: () -> Void

    @State private var isRotating = false

    private var projectName: String {
        projects.first { $0.id == projectId }?.name ?? "Unknown Project"
    }

    private var statusColor: Color {
        switch syncStatus {
        case .inProgress, .UNRECOGNIZED(_):
            return .blue
        case .complete:
            return .green
        case .error:
            return .red
        case .undefined:
            return .gray
        }
    }

    private var statusText: String {
        switch syncStatus {
        case .inProgress, .UNRECOGNIZED(_):
            return "Syncing..."
        case .complete:
            return "Sync completed"
        case .error:
            return "Sync failed"
        case .undefined:
            return "Unknown status"
        }
    }

    private var statusIcon: String {
        switch syncStatus {
        case .inProgress, .UNRECOGNIZED(_):
            return "arrow.triangle.2.circlepath"
        case .complete:
            return "checkmark.circle.fill"
        case .error:
            return "xmark.circle.fill"
        case .undefined:
            return "questionmark.circle.fill"
        }
    }

    var body: some View {
        HStack(spacing: Layout.units(2)) {
            Image(systemName: statusIcon)
                .foregroundColor(statusColor)
                .rotationEffect(.degrees(isRotating ? 360 : 0))
                .animation(
                    isRotating
                        ? Animation.linear(duration: 2.0).repeatForever(autoreverses: false)
                        : .default,
                    value: isRotating
                )
                .onAppear {
                    if syncStatus == .inProgress {
                        isRotating = true
                    }
                }
                .onChange(of: syncStatus) { oldStatus, newStatus in
                    isRotating = newStatus == .inProgress
                }

            VStack(alignment: .leading, spacing: Layout.units(0.5)) {
                Text(projectName)
                    .font(.headline)
                    .foregroundColor(.primary)

                Text(statusText)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button {
                onDismiss()
            } label: {
                Image(systemName: "xmark")
                    .foregroundColor(.secondary)
            }
        }
        .padding(Layout.units(2))
        .background(statusColor.opacity(0.1))
        .cornerRadius(Layout.units(1))
    }
}

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

enum ProjectLocation: Hashable {
    case local(String)
    case cloud(String)

    var id: String {
        switch self {
        case .local(let id), .cloud(let id):
            return id
        }
    }

    var isLocal: Bool {
        switch self {
        case .local: return true
        case .cloud: return false
        }
    }

    var isCloud: Bool {
        switch self {
        case .local: return false
        case .cloud: return true
        }
    }
}

struct ProjectsView: View {
    var projects: [Bloop_ProjectInfo]
    var cloudProjects: [Bloop_ProjectInfo]
    var projectSyncStatuses: [String: Bloop_SyncStatus]
    var dispatch: Dispatch
    var dismiss: () -> Void

    @State private var selected: ProjectLocation?
    @State private var selectedFileURL: URL?

    private var sortedProjects: [Bloop_ProjectInfo] {
        projects.sorted { a, b in
            a.lastSaved > b.lastSaved
        }
    }

    private var sortedCloudProjects: [Bloop_ProjectInfo] {
        cloudProjects.sorted { a, b in
            a.lastSaved > b.lastSaved
        }
    }

    var body: some View {

        NavigationStack {

            VStack(spacing: 0) {
                // Sync notifications
                ForEach(Array(projectSyncStatuses.keys), id: \.self) { projectId in
                    if let syncStatus = projectSyncStatuses[projectId] {
                        ProjectSyncNotificationView(
                            projectId: projectId,
                            syncStatus: syncStatus,
                            projects: projects + cloudProjects
                        ) {
                            dispatch(.dismissProjectSync(projectId))
                        }
                        .padding(.horizontal, Layout.units(2))
                        .padding(.top, Layout.units(1))
                    }
                }

                List(selection: $selected) {
                    if !sortedProjects.isEmpty {
                        Section("Local Projects") {
                            ForEach(sortedProjects) { project in
                                ProjectPreview(
                                    project: project,
                                    selected: selected?.id == project.id
                                        && selected?.isLocal == true
                                )
                                .tag(ProjectLocation.local(project.id))
                                .onTapGesture {
                                    selected = .local(project.id)
                                }
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
                    }

                    if !sortedCloudProjects.isEmpty {
                        Section("Cloud Projects") {
                            ForEach(sortedCloudProjects) { project in
                                ProjectPreview(
                                    project: project,
                                    selected: selected?.id == project.id
                                        && selected?.isCloud == true
                                )
                                .tag(ProjectLocation.cloud(project.id))
                                .onTapGesture {
                                    selected = .cloud(project.id)
                                }
                            }
                        }
                    }
                }
                .listStyle(.plain)
                .navigationTitle("Projects")
                .toolbar {

                    if let selected = selected {

                        if selected.isCloud {
                            Button {
                                dispatch(pullProjectAction(selected.id))
                            } label: {
                                Label("Pull", systemImage: "arrow.down.circle")
                                    .labelStyle(.titleOnly)
                            }
                        }

                        if selected.isLocal {
                            Button {
                                let action = loadProjectAction(selected.id)
                                dispatch(action)
                                dismiss()
                            } label: {
                                Label("Open", systemImage: "folder")
                                    .labelStyle(.titleOnly)
                            }

                            Button {
                                let action = duplicateProjectAction(selected.id)
                                dispatch(action)
                                dismiss()
                            } label: {
                                Label("Duplicate", systemImage: "doc.on.doc")
                                    .labelStyle(.titleOnly)
                            }

                            Button {
                                dispatch(pushProjectAction(selected.id))
                            } label: {
                                Label("Push", systemImage: "arrow.up.circle")
                                    .labelStyle(.titleOnly)
                            }
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
        Group {
            ProjectsView(
                projects: projects,
                cloudProjects: [],
                projectSyncStatuses: [:],
                dispatch: loggingDispatch
            ) {
                print("Dismiss sheet")
            }

            ProjectsView(
                projects: projects,
                cloudProjects: [],
                projectSyncStatuses: [
                    "Project 1": .inProgress,
                    "Project 2": .complete,
                ],
                dispatch: loggingDispatch
            ) {
                print("Dismiss sheet")
            }
        }
    }
}

struct ProjectSyncNotificationView_Previews: PreviewProvider {
    static var previews: some View {
        VStack(spacing: 16) {
            ProjectSyncNotificationView(
                projectId: "1",
                syncStatus: .inProgress,
                projects: [
                    .with {
                        $0.id = "1"
                        $0.name = "Test Project"
                        $0.version = "0"
                        $0.lastSaved = "2025-05-27T15:34:00Z"
                    }
                ]
            ) {
                print("Dismiss in progress")
            }

            ProjectSyncNotificationView(
                projectId: "2",
                syncStatus: .complete,
                projects: [
                    .with {
                        $0.id = "2"
                        $0.name = "Completed Project"
                        $0.version = "0"
                        $0.lastSaved = "2025-05-27T15:34:00Z"
                    }
                ]
            ) {
                print("Dismiss complete")
            }

            ProjectSyncNotificationView(
                projectId: "3",
                syncStatus: .error,
                projects: [
                    .with {
                        $0.id = "3"
                        $0.name = "Failed Project"
                        $0.version = "0"
                        $0.lastSaved = "2025-05-27T15:34:00Z"
                    }
                ]
            ) {
                print("Dismiss error")
            }
        }
        .padding()
    }
}
