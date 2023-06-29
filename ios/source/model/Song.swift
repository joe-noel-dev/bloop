import Foundation

struct Song {
    var id: Id
    var name: String
    var tempo: Tempo
    var sections: [Section]
    var sample: Sample?
}
