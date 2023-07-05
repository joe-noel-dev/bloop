import SwiftUI

struct ProjectsView: View {
    var projects: [ProjectInfo]
    var dispatch: Dispatch
    var dismiss: () -> Void
    
    private var sortedProjects: [ProjectInfo] {
        projects.sorted { a, b in
            a.lastSaved > b.lastSaved
        }
    }

    var body: some View {
        List {
            ForEach(sortedProjects) { project in
                Button {
                    let action = loadProjectAction(project.id)
                    dispatch(action)
                    
                    dismiss()
                } label: {
                    VStack {
                        Text(project.name)
                            .font(.headline)
                        
                        Text("Last saved: \(formatLastSaved(project.lastSaved))")
                            .font(.footnote)
                        
                    }
                }
            }
            .onDelete { offsets in
                offsets.forEach { offset in
                    let action = removeProjectAction(projects[offset].id)
                    dispatch(action)
                }
                
            }
        }
        .onAppear {
            let action = getProjectsAction()
            dispatch(action)
        }
    }
    
    func formatLastSaved(_ millisecondsSince1970: Int64) -> String {
        let secondsSince1970 = millisecondsSince1970 / 1000
        let interval = TimeInterval(secondsSince1970)
        let date = Date(timeIntervalSince1970: interval)
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .full
        return formatter.localizedString(for: date, relativeTo: Date())
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
            lastSaved: Int64((Date() - 37647823).timeIntervalSince1970.magnitude) * 1000
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
