import SwiftUI

struct SectionsView: View {
    var song: Bloop_Song
    var dispatch: Dispatch

    @State private var newSong: Bloop_Song

    init(song: Bloop_Song, dispatch: @escaping Dispatch) {
        self.song = song
        self.dispatch = dispatch
        self.newSong = song
    }

    var body: some View {
        ForEach($newSong.sections, editActions: [.delete, .move]) { section in

            SectionRow(section: section)
                .padding([.top, .bottom], Layout.units(1))
        }
        .navigationTitle(newSong.name)
        .toolbar {
            EditButton()

            Button {
                let newSection = Bloop_Section.with {
                    $0.id = randomId()
                    $0.name = "New Section"
                }

                newSong.sections.append(newSection)
            } label: {
                Label("Add Section", systemImage: "plus")
            }
        }
        .onDisappear {
            if newSong != song {
                let action = updateSongAction(newSong)
                dispatch(action)
            }
        }
    }
}

struct SectionRow: View {

    @Binding var section: Bloop_Section

    var body: some View {

        VStack(alignment: .leading) {

            TextField("Name", text: $section.name)
                .font(.title2)

            HStack {

                Toggle(
                    isOn: $section.metronome,
                    label: {
                        Image(systemName: "metronome")
                    }
                )
                .toggleStyle(.button)

                Toggle(
                    isOn: $section.loop,
                    label: {
                        Image(systemName: "repeat")
                    }
                )
                .toggleStyle(.button)

                TextField("Start", value: $section.start, formatter: NumberFormatter())
                    .textFieldStyle(.roundedBorder)
                    .frame(maxWidth: 64)
                    #if os(iOS)
                        .keyboardType(.decimalPad)
                    #endif

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
