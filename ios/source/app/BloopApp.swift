import SwiftUI

final class AppLifecycleCoordinator: ObservableObject {
    private enum ConnectionTarget {
        case local
        case remote(Server)
    }

    private let backgroundGracePeriod: TimeInterval
    private var target: ConnectionTarget?
    private var backgroundedAt: Date?
    private var pendingShutdown: DispatchWorkItem?
    private var reconnectOnActivation = false

    init(backgroundGracePeriod: TimeInterval = 10 * 60) {
        self.backgroundGracePeriod = backgroundGracePeriod
    }

    func record(_ action: Action) {
        switch action {
        case .connect(let server):
            target = .remote(server)
        case .connectLocal:
            target = .local
        case .disconnect:
            target = nil
            reconnectOnActivation = false
        default:
            break
        }
    }

    func didBecomeActive(store: Store) {
        let wasBackgroundedAt = backgroundedAt
        backgroundedAt = nil
        pendingShutdown?.cancel()
        pendingShutdown = nil

        store.dispatch(.resumeDiscovery)

        if let wasBackgroundedAt,
           Date().timeIntervalSince(wasBackgroundedAt) >= backgroundGracePeriod,
           !reconnectOnActivation {
            disconnect(store: store)
        }

        reconnectIfNeeded(store: store)
    }

    func didEnterBackground(store: Store) {
        backgroundedAt = Date()
        store.dispatch(.pauseDiscovery)

        let shutdown = DispatchWorkItem { [weak self, weak store] in
            guard let self, let store else {
                return
            }
            self.disconnect(store: store)
        }
        pendingShutdown?.cancel()
        pendingShutdown = shutdown
        DispatchQueue.main.asyncAfter(deadline: .now() + backgroundGracePeriod, execute: shutdown)
    }

    func deviceDidLock(store: Store) {
        backgroundedAt = nil
        pendingShutdown?.cancel()
        pendingShutdown = nil
        store.dispatch(.pauseDiscovery)
        disconnect(store: store)
    }

    func willTerminate(store: Store) {
        pendingShutdown?.cancel()
        UIApplication.shared.isIdleTimerDisabled = false
        store.dispatch(.lifecycleDisconnect)
    }

    private func disconnect(store: Store) {
        guard target != nil else {
            return
        }

        reconnectOnActivation = true

        if store.state.playbackState.playing == .playing {
            store.dispatch(stopAction())
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.15) { [weak store] in
                store?.dispatch(.lifecycleDisconnect)
            }
        } else {
            store.dispatch(.lifecycleDisconnect)
        }
    }

    private func reconnectIfNeeded(store: Store) {
        guard reconnectOnActivation, let target else {
            return
        }

        reconnectOnActivation = false
        switch target {
        case .local:
            store.dispatch(.connectLocal)
        case .remote(let server):
            store.dispatch(.connect(server))
        }
    }
}

private final class AppLifecycleMiddleware: Middleware {
    var dispatch: Dispatch?
    private let lifecycle: AppLifecycleCoordinator

    init(lifecycle: AppLifecycleCoordinator) {
        self.lifecycle = lifecycle
    }

    func execute(state: AppState, action: Action) {
        lifecycle.record(action)
    }
}

@main
struct BloopApp: App {

    @AppStorage("appearanceMode") private var appearanceMode = AppearanceMode.system
    @Environment(\.scenePhase) private var scenePhase
    @StateObject private var lifecycle: AppLifecycleCoordinator

    @State private var store: Store

    init() {
        let lifecycle = AppLifecycleCoordinator()
        _lifecycle = StateObject(wrappedValue: lifecycle)
        _store = State(initialValue: Store(
            reducer: rootReducer,
            state: AppState(),
            middlewares: [
                AppLifecycleMiddleware(lifecycle: lifecycle), ApiMiddleware(), UploadMiddleware(),
                AudioSessionMiddleware(), FFIMiddleware(), ApiCodecMiddleware(), ResponseMiddleware(),
            ]
        ))
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
                .preferredColorScheme(appearanceMode.colorScheme)
                .onAppear {
                    lifecycle.didBecomeActive(store: store)
                    updateIdleTimer()
                }
                .onChange(of: scenePhase) {
                    switch scenePhase {
                    case .active:
                        lifecycle.didBecomeActive(store: store)
                    case .background:
                        lifecycle.didEnterBackground(store: store)
                    default:
                        break
                    }
                    updateIdleTimer()
                }
                .onChange(of: store.state.connected) {
                    updateIdleTimer()
                }
                .onReceive(NotificationCenter.default.publisher(
                    for: UIApplication.protectedDataWillBecomeUnavailableNotification
                )) { _ in
                    lifecycle.deviceDidLock(store: store)
                }
                .onReceive(NotificationCenter.default.publisher(
                    for: UIApplication.willTerminateNotification
                )) { _ in
                    lifecycle.willTerminate(store: store)
                }
        }
    }

    private func updateIdleTimer() {
        UIApplication.shared.isIdleTimerDisabled = scenePhase == .active && store.state.connected != nil
    }
}
