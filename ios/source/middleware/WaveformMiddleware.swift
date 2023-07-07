import Foundation

class WaveformMiddleware : Middleware {
    private var waveformIds: Set<Id> = []
    
    func execute(state: AppState, action: Action, dispatch: @escaping Dispatch) {
        if case .setProject(let project) = action {
            let newIds = project.songs.reduce(Set<Id> ()) { (ids, song) in
                var ids = ids
                
                if let sample = song.sample {
                    ids.insert(sample.id)
                }
                
                return ids
            }
            
            let idsToAdd = newIds.subtracting(waveformIds)
            let idsToRemove = waveformIds.subtracting(newIds)
            
            idsToAdd.forEach { id in
                dispatch(getWaveformAction(id))
            }
            
            idsToRemove.forEach { id in
                dispatch (.removeWaveform(id))
            }
            
            waveformIds = newIds
        }
    }
    
    
}
