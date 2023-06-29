import Foundation

protocol Action {}

struct SendRequestAction: Action {
    let request: Request
}

struct SetProjectAction: Action {
    let project: Project
}
