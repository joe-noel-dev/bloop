import Foundation

class WaveformMiddleware: Middleware {
    private var waveformIds: Set<Id> = []
    var dispatch: Dispatch?

    func execute(state: AppState, action: Action) {
        if case .setProject(let project) = action {
            let newIds = project.songs.reduce(Set<Id>()) { (ids, song) in
                var ids = ids

                if song.sample.id != 0 {
                    ids.insert(song.sample.id)
                }

                return ids
            }

            let idsToAdd = newIds.subtracting(waveformIds)
            let idsToRemove = waveformIds.subtracting(newIds)

            for id in idsToAdd {
                self.dispatch?(getWaveformAction(id))
            }

            for id in idsToRemove {
                self.dispatch?(.removeWaveform(id))
            }

            waveformIds = newIds
        }
    }

    func setDispatch(_ dispatch: @escaping Dispatch) {
        self.dispatch = dispatch
    }

}
