import Foundation

func demoProject() -> Project {
    let lastSaved = Int64(Date().timeIntervalSince1970)
    let projectName = "My Project"
    let id = UUID().uuidString
    let version = "1"
    let projectInfo = ProjectInfo.init(
        id: id,
        name: projectName,
        version: version,
        lastSaved: lastSaved
    )

    let songs = Array(0..<3).map { demoSong($0) }
    let selections = Selections(song: songs[0].id, section: songs[0].sections[0].id)

    return Project(info: projectInfo, songs: songs, selections: selections)
}

func demoSample() -> Sample {
    let id = UUID().uuidString
    let name = "Sample"
    let tempo = Tempo(bpm: 124.0)
    let sampleRate = 44100
    let sampleCount = 10 * sampleRate
    let channelCount = 2

    return Sample(
        id: id,
        name: name,
        tempo: tempo,
        sampleRate: sampleRate,
        sampleCount: sampleCount,
        channelCount: channelCount
    )
}

func demoSong(_ index: Int) -> Song {

    let id = UUID().uuidString
    let name = "Song \(index + 1)"
    let tempo = Tempo(bpm: 124.0)
    let sections = Array(0..<8).map { demoSection($0) }
    let sample = demoSample()

    return .init(id: id, name: name, tempo: tempo, sections: sections, sample: sample)
}

func demoSection(_ index: Int) -> Section {
    let duration = 16.0
    let id = UUID().uuidString
    let name = "Section \(index + 1)"
    let start = Double(index) * duration
    let loop = index == 0
    let metronome = index == 1

    return .init(id: id, name: name, start: start, loop: loop, metronome: metronome)
}
