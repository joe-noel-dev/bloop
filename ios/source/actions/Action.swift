import Foundation

enum Action {
    case sendRequest(Request)
    case setProject(Project)
    case connect(String)
    case setConnected(Bool)
}
