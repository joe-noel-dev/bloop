import SwiftUI

struct SectionView: View {
    var section: Section
    var isSelected: Bool
    var dispatch: Dispatch
    @State private var editing = false

    init(section: Section, isSelected: Bool, dispatch: @escaping Dispatch) {
        self.section = section
        self.dispatch = dispatch
        self.isSelected = isSelected
    }

    private var border: some View {
        Rectangle().frame(width: 2).foregroundColor(isSelected ? Colours.theme2 : Colours.neutral6)
    }

    private var editButton: some View {
        Button {
            editing = true
        } label: {
            Label("Edit", systemImage: "pencil")
        }
        .buttonStyle(.bordered)
        .popover(isPresented: $editing) {
            EditSectionView(section: section, dispatch: dispatch)
                .presentationDetents([.medium])
        }
    }

    private var statusIcons: some View {
        HStack {
            if section.loop {
                Image(systemName: "repeat")
            }

            if section.metronome {
                Image(systemName: "metronome")
            }
        }
    }

    var body: some View {
        HStack {
            Text(section.name)
                .font(.title2)

            Spacer()

            statusIcons

            if isSelected {
                editButton
            }

        }
        .contentShape(Rectangle())
        .padding([.leading])
        .padding([.top, .bottom], Layout.units(0.5))
        .overlay(border, alignment: .leading)
        .frame(minHeight: Layout.touchTarget)
        .onTapGesture {
            let action = selectSectionAction(section.id)
            dispatch(action)
        }

    }
}

struct SectionView_Previews: PreviewProvider {
    static var section: Section {
        let section = demoSection(0)
        return section
    }

    static var previews: some View {
        SectionView(section: section, isSelected: true) { action in
            print("Dipatch: \(action)")
        }
        .padding()
    }
}
