import Foundation

protocol Middleware {
    mutating func setDispatch(_ dispatch: @escaping Dispatch)
    mutating func execute(state: AppState, action: Action)
}
