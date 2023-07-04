import Foundation

struct Selections: Codable {
    var song: Id?
    var section: Id?

    func isSongSelected(songId: Id) -> Bool {
        song == songId
    }

    func isSectionSelected(sectionId: Id) -> Bool {
        section == sectionId
    }
}
