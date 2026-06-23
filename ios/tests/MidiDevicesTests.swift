import XCTest

@testable import Bloop

final class MidiDevicesTests: XCTestCase {

    func testSetMidiDevicesAction() {
        let initialState = AppState()
        let devices = Bloop_MidiDevices.with {
            $0.portNames = ["iCON G_Boar V1.03", "USB MIDI Interface"]
        }

        let action = Action.setMidiDevices(devices)
        let newState = rootReducer(state: initialState, action: action)

        XCTAssertNotNil(newState.midiDevices)
        XCTAssertEqual(newState.midiDevices?.portNames.count, 2)
        XCTAssertEqual(newState.midiDevices?.portNames.first, "iCON G_Boar V1.03")
    }

    func testGetMidiDevicesActionCreation() {
        let action = getMidiDevicesAction()

        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasGet)
            XCTAssertEqual(request.get.entity, .midiDevices)
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testResponseMiddlewareDispatchesMidiDevices() {
        var dispatched: [Action] = []
        let middleware = ResponseMiddleware()
        middleware.dispatch = { dispatched.append($0) }

        let response = Bloop_Response.with {
            $0.midiDevices = Bloop_MidiDevices.with {
                $0.portNames = ["iCON G_Boar V1.03"]
            }
        }

        middleware.execute(state: AppState(), action: .receivedResponse(response))

        XCTAssertTrue(dispatched.contains(where: {
            if case .setMidiDevices(let d) = $0 { return d.portNames == ["iCON G_Boar V1.03"] }
            return false
        }))
    }

    func testMidiPreferencesTogglesAddPort() {
        var editedPreferences = Bloop_Preferences()
        let portName = "iCON G_Boar V1.03"

        XCTAssertFalse(editedPreferences.midi.enabledDevices.contains(portName))

        if !editedPreferences.midi.enabledDevices.contains(portName) {
            editedPreferences.midi.enabledDevices.append(portName)
        }

        XCTAssertTrue(editedPreferences.midi.enabledDevices.contains(portName))
        XCTAssertEqual(editedPreferences.midi.enabledDevices.count, 1)
    }

    func testMidiPreferencesTogglesRemovePort() {
        var editedPreferences = Bloop_Preferences.with {
            $0.midi = Bloop_MidiPreferences.with {
                $0.enabledDevices = ["iCON G_Boar V1.03", "USB MIDI Interface"]
            }
        }

        editedPreferences.midi.enabledDevices.removeAll { $0 == "iCON G_Boar V1.03" }

        XCTAssertEqual(editedPreferences.midi.enabledDevices, ["USB MIDI Interface"])
    }

    func testUpdatePreferencesActionWithTwoEnabledDevices() {
        let preferences = Bloop_Preferences.with {
            $0.midi = Bloop_MidiPreferences.with {
                $0.enabledDevices = ["iCON G_Boar V1.03", "USB MIDI Interface"]
            }
        }

        let action = updatePreferencesAction(preferences)

        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasUpdate)
            XCTAssertTrue(request.update.hasPreferences)
            XCTAssertEqual(request.update.preferences.midi.enabledDevices, ["iCON G_Boar V1.03", "USB MIDI Interface"])
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testMidiDevicesTwoPortNamesAreStoredInState() {
        let initialState = AppState()
        let devices = Bloop_MidiDevices.with {
            $0.portNames = ["iCON G_Boar V1.03", "USB MIDI Interface"]
        }

        let newState = rootReducer(state: initialState, action: .setMidiDevices(devices))

        XCTAssertEqual(newState.midiDevices?.portNames.count, 2)
        XCTAssertEqual(newState.midiDevices?.portNames, ["iCON G_Boar V1.03", "USB MIDI Interface"])
    }
}
