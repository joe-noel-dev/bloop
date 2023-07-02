import SwiftUI

@main
struct BloopApp: App {

    @State var store = Store(
        reducer: rootReducer, state: AppState(), middlewares: [ApiMiddleware()])

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
                .onAppear {
                    store.dispatch(.connect)
                }

        }
    }
}
