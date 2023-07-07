import SwiftUI

@main
struct BloopApp: App {

    static private let defaultIpAddress = "localhost"

    @State var store = Store(
        reducer: rootReducer,
        state: AppState(),
        middlewares: [ApiMiddleware(), WaveformMiddleware()]
    )

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
                .onAppear {
                    store.dispatch(.connect(BloopApp.defaultIpAddress))
                }

        }
    }
}
