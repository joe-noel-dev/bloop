import Foundation

func updateSectionAction(_ section: Section) -> Action {
    let updateRequest = UpdateRequest.section(section)
    let request = Request.update(updateRequest)
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
