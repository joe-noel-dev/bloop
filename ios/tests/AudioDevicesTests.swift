import XCTest

@testable import Bloop

final class AudioDevicesTests: XCTestCase {

    func testSetAudioDevicesAction() {
        let initialState = AppState()
        let devices = Bloop_AudioDevices.with {
            $0.hostName = "CoreAudio"
            $0.devices = [
                Bloop_AudioDevice.with {
                    $0.id = "device-1"
                    $0.name = "Built-in Output"
                    $0.isDefault = true
                    $0.supportedSampleRates = [44100, 48000]
                    $0.supportedChannelCounts = [2]
                }
            ]
        }

        let action = Action.setAudioDevices(devices)
        let newState = rootReducer(state: initialState, action: action)

        XCTAssertNotNil(newState.audioDevices)
        XCTAssertEqual(newState.audioDevices?.hostName, "CoreAudio")
        XCTAssertEqual(newState.audioDevices?.devices.count, 1)
        XCTAssertEqual(newState.audioDevices?.devices.first?.id, "device-1")
        XCTAssertEqual(newState.audioDevices?.devices.first?.name, "Built-in Output")
        XCTAssertTrue(newState.audioDevices?.devices.first?.isDefault ?? false)
    }

    func testSetAudioStatusRunning() {
        let initialState = AppState()
        let status = Bloop_AudioStatus.with {
            $0.currentDeviceID = "device-1"
            $0.currentDeviceName = "Built-in Output"
            $0.currentSampleRate = 48000
            $0.currentChannelCount = 2
            $0.currentBufferSize = 512
            $0.engineStatus = .running
        }

        let action = Action.setAudioStatus(status)
        let newState = rootReducer(state: initialState, action: action)

        XCTAssertNotNil(newState.audioStatus)
        XCTAssertEqual(newState.audioStatus?.currentDeviceID, "device-1")
        XCTAssertEqual(newState.audioStatus?.currentDeviceName, "Built-in Output")
        XCTAssertEqual(newState.audioStatus?.currentSampleRate, 48000)
        XCTAssertEqual(newState.audioStatus?.currentChannelCount, 2)
        XCTAssertEqual(newState.audioStatus?.engineStatus, .running)
    }

    func testSetAudioStatusFailed() {
        let initialState = AppState()
        let status = Bloop_AudioStatus.with {
            $0.engineStatus = .failed
            $0.error = "Device not found"
        }

        let action = Action.setAudioStatus(status)
        let newState = rootReducer(state: initialState, action: action)

        XCTAssertEqual(newState.audioStatus?.engineStatus, .failed)
        XCTAssertEqual(newState.audioStatus?.error, "Device not found")
    }

    func testGetAudioDevicesActionCreation() {
        let action = getAudioDevicesAction()

        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasGet)
            XCTAssertEqual(request.get.entity, .audioDevices)
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testAudioControlRestartActionCreation() {
        let action = audioControlAction(method: .restart)

        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasAudioControl)
            XCTAssertEqual(request.audioControl.method, .restart)
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testAudioControlStartActionCreation() {
        let action = audioControlAction(method: .start)

        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasAudioControl)
            XCTAssertEqual(request.audioControl.method, .start)
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testResponseMiddlewareDispatchesAudioDevices() {
        var dispatched: [Action] = []
        let middleware = ResponseMiddleware()
        middleware.dispatch = { dispatched.append($0) }

        let response = Bloop_Response.with {
            $0.audioDevices = Bloop_AudioDevices.with {
                $0.hostName = "CoreAudio"
                $0.devices = []
            }
        }

        middleware.execute(state: AppState(), action: .receivedResponse(response))

        XCTAssertTrue(dispatched.contains(where: {
            if case .setAudioDevices(let d) = $0 { return d.hostName == "CoreAudio" }
            return false
        }))
    }

    func testResponseMiddlewareDispatchesAudioStatus() {
        var dispatched: [Action] = []
        let middleware = ResponseMiddleware()
        middleware.dispatch = { dispatched.append($0) }

        let response = Bloop_Response.with {
            $0.audioStatus = Bloop_AudioStatus.with {
                $0.engineStatus = .running
                $0.currentSampleRate = 44100
            }
        }

        middleware.execute(state: AppState(), action: .receivedResponse(response))

        XCTAssertTrue(dispatched.contains(where: {
            if case .setAudioStatus(let s) = $0 { return s.engineStatus == .running }
            return false
        }))
    }
}
