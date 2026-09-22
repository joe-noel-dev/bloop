//
//  BloopTests.swift
//  BloopTests
//
//  Created by Joe Noel on 29/06/2023.
//

import XCTest

@testable import Bloop

final class BloopTests: XCTestCase {

    func testDeviceLockDisconnectsAndReconnectsLastLocalTarget() {
        let capture = ActionCaptureMiddleware()
        let store = makeStore(capture: capture)
        let lifecycle = AppLifecycleCoordinator()

        lifecycle.record(.connectLocal)
        lifecycle.deviceDidLock(store: store)
        flushStore()

        XCTAssertTrue(capture.containsLifecycleDisconnect)

        lifecycle.didBecomeActive(store: store)
        flushStore()

        XCTAssertTrue(capture.containsConnectLocal)
    }

    func testBackgroundGraceExpiryDisconnectsAndReconnects() {
        let capture = ActionCaptureMiddleware()
        let store = makeStore(capture: capture)
        let lifecycle = AppLifecycleCoordinator(backgroundGracePeriod: 0.01)

        lifecycle.record(.connectLocal)
        lifecycle.didEnterBackground(store: store)

        let graceExpired = expectation(description: "background grace expires")
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
            graceExpired.fulfill()
        }
        wait(for: [graceExpired], timeout: 1)
        flushStore()

        XCTAssertTrue(capture.containsLifecycleDisconnect)

        lifecycle.didBecomeActive(store: store)
        flushStore()

        XCTAssertTrue(capture.containsConnectLocal)
    }

    func testManualDisconnectSuppressesReconnect() {
        let capture = ActionCaptureMiddleware()
        let store = makeStore(capture: capture)
        let lifecycle = AppLifecycleCoordinator()

        lifecycle.record(.connectLocal)
        lifecycle.deviceDidLock(store: store)
        lifecycle.record(.disconnect)
        lifecycle.didBecomeActive(store: store)
        flushStore()

        XCTAssertFalse(capture.containsConnectLocal)
    }

    private func makeStore(capture: ActionCaptureMiddleware) -> Store {
        Store(reducer: rootReducer, state: AppState(), middlewares: [capture])
    }

    private func flushStore() {
        let flushed = expectation(description: "store queue flushes")
        DispatchQueue.main.async {
            flushed.fulfill()
        }
        wait(for: [flushed], timeout: 1)
    }

}

private final class ActionCaptureMiddleware: Middleware {
    var dispatch: Dispatch?
    private(set) var actions: [Action] = []

    func execute(state: AppState, action: Action) {
        actions.append(action)
    }

    var containsLifecycleDisconnect: Bool {
        actions.contains { action in
            if case .lifecycleDisconnect = action {
                return true
            }
            return false
        }
    }

    var containsConnectLocal: Bool {
        actions.contains { action in
            if case .connectLocal = action {
                return true
            }
            return false
        }
    }
}
