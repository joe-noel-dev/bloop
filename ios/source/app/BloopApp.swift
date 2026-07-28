import SwiftUI

@main
struct BloopApp: App {

    @AppStorage("appearanceMode") private var appearanceMode = AppearanceMode.system

    @State var store = Store(
        reducer: rootReducer,
        state: AppState(),
        middlewares: [
            ApiMiddleware(), UploadMiddleware(), AudioSessionMiddleware(),
            FFIMiddleware(), ApiCodecMiddleware(), ResponseMiddleware(),
        ]
    )

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
                .preferredColorScheme(appearanceMode.colorScheme)
                .onAppear {
                    UIApplication.shared.isIdleTimerDisabled = true
                }
        }
    }
}
