import SwiftUI

struct SectionView: View {
    var section: Section
    var dispatch: (Action) -> Void
    private var loopBinding: Binding<Bool>
    private var metronomeBinding: Binding<Bool>
    @State private var newStart: Double
    @State private var newName: String

    init(section: Section, dispatch: @escaping (Action) -> Void) {
        self.section = section
        self.dispatch = dispatch

        self.loopBinding = .init(
            get: {
                section.loop
            },
            set: {
                var section = section
                section.loop = $0
                updateSection(section: section, dispatch: dispatch)
            })

        self.metronomeBinding = .init(
            get: {
                section.metronome
            },
            set: {
                var section = section
                section.metronome = $0
                updateSection(section: section, dispatch: dispatch)
            })

        self.newStart = section.start
        self.newName = section.name
    }

    var body: some View {
        HStack {
            TextField("Name", text: $newName, onCommit: {
                var section = section
                section.name = newName
                updateSection(section: section, dispatch: dispatch)
            })
                .font(.title2)

            HStack {
                Text("Start")
                TextField("Start", value: $newStart, format: .number)
                    .onSubmit {
                        var section = section
                        section.start = newStart
                        updateSection(section: section, dispatch: dispatch)
                    }
                    .frame(maxWidth: 48)
                    .keyboardType(.numberPad)
            }

            Toggle(isOn: loopBinding) {
                Label("Loop", systemImage: "repeat")
            }
            .toggleStyle(.button)

            Toggle(isOn: metronomeBinding) {
                Label("Metronome", systemImage: "metronome")
            }
            .toggleStyle(.button)
        }
    }
}

struct SectionView_Previews: PreviewProvider {
    static var section: Section {
        let section = demoSection(0)
        return section
    }

    static var previews: some View {
        SectionView(section: section) { action in
            print("Dipatch: \(action)")
        }
            .padding()
    }
}

func updateSection(section: Section, dispatch: (Action) -> Void) {
    let updateRequest = UpdateRequest.section(section)
    let request = Request.update(updateRequest)
    let action = Action.sendRequest(request)
    dispatch(action)
}
