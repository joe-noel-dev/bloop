import Foundation

struct Selections: Codable, Equatable {
    var song: Id?
    var section: Id?

    func isSongSelected(songId: Id) -> Bool {
        song == songId
    }

    func isSectionSelected(sectionId: Id) -> Bool {
        section == sectionId
    }
}
