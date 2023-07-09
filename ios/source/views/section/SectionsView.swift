import SwiftUI

struct SectionsView: View {
    var song: Song
    var dispatch: Dispatch

    init(song: Song, dispatch: @escaping Dispatch) {
        self.song = song
        self.dispatch = dispatch
    }

    var body: some View {
        NavigationView {
            List {

                ForEach(song.sections) { section in
                    SectionRow(section: section, dispatch: dispatch)
                        .padding([.top, .bottom], Layout.units(1))
                }
                .onDelete(perform: { values in
                    values.map { index in
                        song.sections[index].id
                    }.forEach { sectionId in
                        let action = removeSectionAction(sectionId)
                        dispatch(action)
                    }
                })

            }
            .navigationTitle(song.name)
            .toolbar {
                Button {
                    let action = addSectionAction(song.id)
                    dispatch(action)
                } label: {
                    Label("Add Section", systemImage: "plus")
                }
            }
        }
    }
}

struct SectionRow: View {
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
    
    private func updateSection(_ newSection: Section) {
        let action = updateSectionAction(newSection)
        dispatch(action)
    }

    var body: some View {

        HStack {

            TextField("Name", text: $newName)
                .font(.title2)
                .onSubmit {
                    var newSection = section
                    newSection.name = newName
                    updateSection(newSection)
                }

            Spacer()

            Toggle(
                isOn: .init(get: {
                    section.metronome
                }, set: { value in
                    var newSection = section
                    newSection.metronome = value
                    updateSection(newSection)
                }),
                label: {
                    Image(systemName: "metronome")
                }
            )
            .toggleStyle(.button)

            Toggle(isOn: .init(get: {
                section.loop
            }, set: { value in
                var newSection = section
                newSection.loop = value
                updateSection(newSection)
            })) {
                Image(systemName: "repeat")
            }
            .toggleStyle(.button)

            TextField("Start", value: $newStart, formatter: NumberFormatter())
                .textFieldStyle(.roundedBorder)
                .frame(maxWidth: 64)
                #if os(iOS)
                    .keyboardType(.decimalPad)
                #endif
                .onSubmit {
                    var newSection = section
                    newSection.name = newName
                    updateSection(newSection)
                }
        }

    }
}

struct SectionsView_Previews: PreviewProvider {
    static let song = {
        let song = demoSong(0)
        return song
    }()

    static var previews: some View {
        SectionsView(song: song, dispatch: loggingDispatch)
    }
}
