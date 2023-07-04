import SwiftUI

struct EditSectionView: View {
    var section: Section
    var dispatch: Dispatch

    @State private var newStart: Double
    @State private var newName: String

    init(section: Section, dispatch: @escaping Dispatch) {
        self.section = section
        self.newStart = section.start
        self.newName = section.name
        self.dispatch = dispatch
    }

    private var loopToggle: some View {
        Toggle(
            isOn: .init(
                get: {
                    section.loop
                },
                set: {
                    var section = section
                    section.loop = $0
                    updateSection(section)
                }
            )
        ) {
            Label("Loop", systemImage: "repeat")
        }
        .toggleStyle(.button)
    }

    private var metronomeToggle: some View {
        Toggle(
            isOn: .init(
                get: {
                    section.metronome
                },
                set: {
                    var section = section
                    section.metronome = $0
                    updateSection(section)
                }
            )
        ) {
            Label("Metronome", systemImage: "metronome")
        }
        .toggleStyle(.button)
    }

    private var startField: some View {
        HStack {
            Text("Start: ")

            Spacer()

            TextField("Start", value: $newStart, format: .number)
                .onSubmit {
                    var section = section
                    section.start = newStart
                    updateSection(section)
                }
                #if os(iOS)
                    .keyboardType(.numberPad)
                #endif
        }
    }

    private func updateSection(_ section: Section) {
        let action = updateSectionAction(section)
        dispatch(action)
    }

    private var nameField: some View {
        TextField("Name", text: $newName)
            .font(.title)
            .onSubmit {
                var section = section
                section.name = newName
                updateSection(section)
            }
    }

    private var removeButton: some View {
        Button {
            let action = removeSectionAction(section.id)
            dispatch(action)
        } label: {
            Label("Remove", systemImage: "trash")
        }
        .buttonStyle(.bordered)
    }

    var body: some View {
        VStack(alignment: .leading) {
            nameField
            startField
            loopToggle
            metronomeToggle
            removeButton
        }
        .padding(Layout.units(2))
    }
}

struct EditSectionView_Previews: PreviewProvider {
    static let section = {
        let section = demoSection(0)
        return section
    }()
    static var previews: some View {
        EditSectionView(section: section) { action in
            print("Dispatch: \(action)")
        }
    }
}
