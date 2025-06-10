import Foundation
import Network

typealias Server = NWEndpoint

typealias Id = UInt64

func selectedSongIndex(_ project: Bloop_Project) -> Int? {
    let songId = project.selections.song
    let songs = project.songs
    return songs.firstIndex { song in
        song.id == songId
    }
}

func songWithOffsetId(_ project: Bloop_Project, offset: Int) -> Id? {
    guard let index = selectedSongIndex(project) else {
        return nil
    }

    let songCount = project.songs.count
    let newIndex = index + offset
    guard 0 <= newIndex && newIndex < songCount else {
        return nil
    }

    return project.songs[newIndex].id
}

func nextSongId(_ project: Bloop_Project) -> Id? {
    songWithOffsetId(project, offset: 1)
}

func previousSongId(_ project: Bloop_Project) -> Id? {
    songWithOffsetId(project, offset: -1)
}

private func sectionWithOffset(_ project: Bloop_Project, songId: Id, sectionId: Id, offset: Int)
    -> Bloop_Section?
{
    guard let song = project.songs.first(where: { $0.id == songId }),
        let sectionIndex = song.sections.firstIndex(where: { $0.id == sectionId })
    else {
        return nil
    }

    let newIndex = sectionIndex + offset
    guard song.sections.indices.contains(newIndex) else {
        return nil
    }

    return song.sections[newIndex]
}

func previousSection(_ project: Bloop_Project, songId: Id, sectionId: Id) -> Bloop_Section? {
    return sectionWithOffset(project, songId: songId, sectionId: sectionId, offset: -1)
}

func nextSection(_ project: Bloop_Project, songId: Id, sectionId: Id) -> Bloop_Section? {
    return sectionWithOffset(project, songId: songId, sectionId: sectionId, offset: 1)
}

func isSongSelected(selections: Bloop_Selections, songId: UInt64) -> Bool {
    selections.song == songId
}

func isSectionSelected(selections: Bloop_Selections, sectionId: Id) -> Bool {
    selections.section == sectionId
}

func randomId() -> Id {
    UInt64.random(in: UInt64.min...UInt64.max)
}

func demoProject() -> Bloop_Project {
    let songs = Array(0..<3).map { demoSong($0) }

    let selections = Bloop_Selections.with {
        $0.song = songs[0].id
        $0.section = songs[0].sections[0].id
    }

    return Bloop_Project.with {
        $0.songs = songs
        $0.selections = selections
    }
}

func demoSample() -> Bloop_Sample {
    .with {
        $0.id = randomId()
        $0.name = "Sample"
        $0.tempo = Bloop_Tempo.with { $0.bpm = 124.0 }
        $0.sampleRate = 44100
        $0.sampleCount = Int64(10 * $0.sampleRate)
        $0.channelCount = 2
    }
}

func demoSong(_ index: Int) -> Bloop_Song {
    .with {
        $0.id = randomId()
        $0.name = "Song \(index + 1)"
        $0.tempo = .with {
            $0.bpm = 124.0
        }
        $0.sections = Array(0..<8).map { demoSection($0) }
        $0.sample = demoSample()
    }
}

func demoSection(_ index: Int) -> Bloop_Section {
    let duration = 16.0

    return Bloop_Section.with {
        $0.id = randomId()
        $0.name = "Section \(index + 1)"
        $0.start = Double(index) * duration
        $0.loop = index == 0
        $0.metronome = index == 1
    }
}

extension Bloop_Song: Identifiable {}
extension Bloop_Section: Identifiable {}
extension Bloop_Sample: Identifiable {}
extension Bloop_ProjectInfo: Identifiable {}
