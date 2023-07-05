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
    }

    private var startField: some View {
        TextField("Start", value: $newStart, format: .number)
            .onSubmit {
                var section = section
                section.start = newStart
                updateSection(section)
            }
            #if os(iOS)
                .keyboardType(.decimalPad)
            #endif
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
        Button(role: .destructive) {
            let action = removeSectionAction(section.id)
            dispatch(action)
        } label: {
            Text("Remove Section")
        }
    }

    var body: some View {
        Form {
            SwiftUI.Section {
                nameField
            }

            SwiftUI.Section {
                startField
            } header: {
                Text("Beat offset")
            }

            SwiftUI.Section {
                loopToggle
                metronomeToggle
            }

            removeButton
        }
        .frame(minWidth: 400, minHeight: 400)
    }
}

struct EditSectionView_Previews: PreviewProvider {
    static let section = {
        let section = demoSection(0)
        return section
    }()
    static var previews: some View {
        EditSectionView(section: section, dispatch: loggingDispatch)
    }
}
