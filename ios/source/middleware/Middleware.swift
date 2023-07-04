import Foundation

protocol Middleware {
    mutating func execute(state: AppState, action: Action, dispatch: @escaping Dispatch)
}
