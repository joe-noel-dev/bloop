import Foundation

enum Action {
    case sendRequest(Request)
    case setProject(Project)
    case connect
    case setConnected(Bool)
}
