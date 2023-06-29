import SwiftUI

let store = Store(reducer: rootReducer, state: AppState(), middlewares: [])

@main
struct BloopApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView().environmentObject(store)
        }
    }
}
