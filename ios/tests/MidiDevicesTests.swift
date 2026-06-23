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
}
