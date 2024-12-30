import Foundation

protocol Middleware {
    var dispatch: Dispatch? { get set }

    func execute(state: AppState, action: Action)
}
