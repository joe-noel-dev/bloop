import SwiftUI

struct SectionsView: View {
    var song: Song
    var dispatch: Dispatch
    @State var newSong: Song

    init(song: Song, dispatch: @escaping Dispatch) {
        self.song = song
        self.dispatch = dispatch
        self.newSong = song
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
    var newSection: Binding<Section>
    var dispatch: Dispatch

    @State private var newStart: Double
    @State private var newName: String

    init(section: Section, dispatch: @escaping Dispatch) {
        self.section = section

        self.newSection = .init(
            get: { section },
            set: { value in
                let action = updateSectionAction(value)
                dispatch(action)
            }
        )

        self.newStart = section.start
        self.newName = section.name
        self.dispatch = dispatch
    }

    var body: some View {

        HStack {

            TextField("Name", text: $newName)
                .font(.title2)
                .onSubmit {
                    newSection.wrappedValue.name = newName
                }

            Spacer()

            Toggle(
                isOn: newSection.metronome,
                label: {
                    Image(systemName: "metronome")
                }
            )
            .toggleStyle(.button)

            Toggle(isOn: newSection.loop) {
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
                    newSection.wrappedValue.start = newStart
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
