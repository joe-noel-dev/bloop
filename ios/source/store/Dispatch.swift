import Foundation

typealias Dispatch = (Action) -> Void

func loggingDispatch(_ action: Action) {
    print("Dispatch: \(action)")
}
