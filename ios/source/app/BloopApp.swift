import SwiftUI

@main
struct BloopApp: App {
    
    @StateObject private var lifecycleHandler = AppLifecycleHandler()

    @State var store = Store(
        reducer: rootReducer,
        state: AppState(),
        middlewares: [ApiMiddleware(), WaveformMiddleware(), UploadMiddleware()]
    )
    
    init() {
        lifecycleHandler.preventSleep = true
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
                .environmentObject(lifecycleHandler)

        }
    }
}
 
class AppLifecycleHandler: ObservableObject {
    @Published var preventSleep: Bool = false {
        didSet {
            UIApplication.shared.isIdleTimerDisabled = preventSleep
        }
    }
}
