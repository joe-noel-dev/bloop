import SwiftUI

struct SectionView: View {
    var section: Section
    var dispatch: (Action) -> Void
    private var loopBinding: Binding<Bool>
    private var metronomeBinding: Binding<Bool>
    private var startBinding: Binding<Double>

    init(section: Section, dispatch: @escaping (Action) -> Void) {
        self.section = section
        self.dispatch = dispatch

        self.loopBinding = .init(
            get: {
                section.loop
            },
            set: {
                var newSection = section
                newSection.loop = $0
                updateSection(dispatch: dispatch, newSection: newSection)
            })

        self.metronomeBinding = .init(
            get: {
                section.metronome
            },
            set: {
                var newSection = section
                newSection.metronome = $0
                updateSection(dispatch: dispatch, newSection: newSection)
            })

        self.startBinding = .init(
            get: {
                section.start
            },
            set: {
                var newSection = section
                newSection.start = $0
                updateSection(dispatch: dispatch, newSection: newSection)
            })

    }

    var body: some View {
        VStack(alignment: .leading) {
            Text(section.name)
                .font(.title)

            Toggle(isOn: loopBinding) {
                Text("Loop")
            }

            Toggle(isOn: metronomeBinding) {
                Text("Metronome")
            }

            HStack {
                Text("Start")
                Spacer()
                TextField("Start", value: startBinding, format: .number)
                    .frame(maxWidth: 48)
            }

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
    }
}

func updateSection(dispatch: (Action) -> Void, newSection: Section) {
    let updateRequest = UpdateRequest.section(newSection)
    let request = Request.update(updateRequest)
    let action = Action.sendRequest(request)
    dispatch(action)
}
