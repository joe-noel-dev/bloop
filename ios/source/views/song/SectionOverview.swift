import SwiftUI

struct SectionOverview: View {
    var section: Section
    var selections: Selections
    var dispatch: Dispatch

    var isSelected: Bool {
        selections.section == section.id
    }

    var entityId: EntityId {
        .init(entity: .section, id: section.id)
    }

    func selectSection() {
        let request = Request.select(entityId)
        dispatch(.sendRequest(request))
    }

    func removeSection() {
        let request = Request.remove(entityId)
        dispatch(.sendRequest(request))
    }

    var body: some View {
        Text(section.name)
            .padding()
            .background(isSelected ? Colours.theme3 : Colours.neutral2)
            .onTapGesture {
                selectSection()
            }
            .contextMenu {
                Button {
                    removeSection()
                } label: {
                    Label("Remove Section", systemImage: "trash")
                }
            }
            .cornerRadius(Layout.corderRadiusMedium)
    }
}

struct SectionOverview_Previews: PreviewProvider {

    static var section: Section {
        var section = demoSection(0)
        section.name = "Chorus"
        return section
    }

    static var selections: Selections {
        .init(section: section.id)
    }

    static var previews: some View {
        SectionOverview(section: section, selections: selections) { action in
            print("Dispatch: \(action)")
        }
        .previewLayout(.sizeThatFits)
        .padding()
    }
}
