import SwiftUI

struct EditSectionView: View {
    var section: Section
    var dispatch: Dispatch

    @State private var newSection: Section

    init(section: Section, dispatch: @escaping Dispatch) {
        self.section = section
        self.newSection = section
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
        TextField("Start", value: $newSection.start, format: .number)
            .onSubmit {
                updateSection(newSection)
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
        TextField("Name", text: $newSection.name)
            .font(.title)
            .onSubmit {
                updateSection(newSection)
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

            SwiftUI.Section(header: Text("Beat Start")) {
                startField
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
