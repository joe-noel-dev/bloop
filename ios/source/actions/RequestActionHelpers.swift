import Foundation

func updateSectionAction(_ section: Section) -> Action {
    let updateRequest = UpdateRequest.section(section)
    let request = Request.update(updateRequest)
    return .sendRequest(request)
}

func updateSongAction(_ song: Song) -> Action {
    let updateRequest = UpdateRequest.song(song)
    let request = Request.update(updateRequest)
    return .sendRequest(request)
}

func renameProjectAction(_ name: String) -> Action {
    let renameRequest = RenameRequest(entity: .project, name: name)
    let request = Request.rename(renameRequest)
    return .sendRequest(request)
}

func selectAction(_ entityId: EntityId) -> Action {
    let request = Request.select(entityId)
    return .sendRequest(request)
}

func selectSectionAction(_ sectionId: Id) -> Action {
    let entity = EntityId.init(entity: .section, id: sectionId)
    return selectAction(entity)
}

func selectSongAction(_ songId: Id) -> Action {
    let entity = EntityId.init(entity: .song, id: songId)
    return selectAction(entity)
}

func removeAction(_ entityId: EntityId) -> Action {
    let request = Request.remove(entityId)
    return .sendRequest(request)
}

func removeSongAction(_ songId: Id) -> Action {
    let entity = EntityId.init(entity: .song, id: songId)
    return removeAction(entity)
}

func removeSectionAction(_ sectionId: Id) -> Action {
    let entity = EntityId.init(entity: .section, id: sectionId)
    return removeAction(entity)
}

func stopAction() -> Action {
    let transportRequest = TransportRequest.stop
    let request = Request.transport(transportRequest)
    return .sendRequest(request)
}

func playAction() -> Action {
    let transportRequest = TransportRequest.play
    let request = Request.transport(transportRequest)
    return .sendRequest(request)
}

func enterLoopAction() -> Action {
    let transportRequest = TransportRequest.loop
    let request = Request.transport(transportRequest)
    return .sendRequest(request)
}

func exitLoopAction() -> Action {
    let transportRequest = TransportRequest.exitLoop
    let request = Request.transport(transportRequest)
    return .sendRequest(request)
}

func queueAction(song: Id, section: Id) -> Action {
    let queueRequest = QueueRequest.init(songId: song, sectionId: section)
    let transportRequest = TransportRequest.queue(queueRequest)
    let request = Request.transport(transportRequest)
    return .sendRequest(request)
}

func addSongAction() -> Action {
    let entity = EntityId.init(entity: .song)
    let request = Request.add(entity)
    return .sendRequest(request)
}

func addSectionAction(_ songId: Id) -> Action {
    let entity = EntityId.init(entity: .section, id: songId)
    let request = Request.add(entity)
    return .sendRequest(request)
}

func getProjectsAction() -> Action {
    let entity = EntityId.init(entity: .projects)
    let request = Request.get(entity)
    return .sendRequest(request)
}

func removeProjectAction(_ projectId: Id) -> Action {
    let entity = EntityId.init(entity: .project, id: projectId)
    return removeAction(entity)
}

func loadProjectAction(_ projectId: Id) -> Action {
    let loadRequest = LoadRequest.init(id: projectId)
    let request = Request.load(loadRequest)
    return .sendRequest(request)
}

func newProjectAction() -> Action {
    let entity = EntityId.init(entity: .project)
    let request = Request.add(entity)
    return .sendRequest(request)
}

func duplicateProjectAction(_ projectId: Id) -> Action {
    let entity = EntityId.init(entity: .project, id: projectId)
    let request = Request.duplicate(entity)
    return .sendRequest(request)
}

func getWaveformAction(_ sampleId: Id) -> Action {
    let entity = EntityId.init(entity: .waveform, id: sampleId)
    let request = Request.get(entity)
    return .sendRequest(request)
}
