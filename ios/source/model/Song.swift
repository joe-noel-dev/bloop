import Foundation

struct Song: Codable, Equatable, Hashable, Identifiable {
    var id: Id
    var name: String
    var tempo: Tempo
    var sections: [Section]
    var sample: Sample?
}
