import XCTest

@testable import Bloop

final class PreferencesTests: XCTestCase {

    func testSetPreferencesAction() {
        // Given
        let initialState = AppState()
        let preferences = Bloop_Preferences.with {
            $0.audio = Bloop_AudioPreferences.with {
                $0.outputDevice = "Test Device"
                $0.sampleRate = 48000
                $0.bufferSize = 512
            }
            $0.midi = Bloop_MidiPreferences.with {
                $0.inputDevice = "Test MIDI"
            }
        }

        // When
        let action = Action.setPreferences(preferences)
        let newState = rootReducer(state: initialState, action: action)

        // Then
        XCTAssertNotNil(newState.preferences)
        XCTAssertEqual(newState.preferences?.audio.outputDevice, "Test Device")
        XCTAssertEqual(newState.preferences?.audio.sampleRate, 48000)
        XCTAssertEqual(newState.preferences?.audio.bufferSize, 512)
        XCTAssertEqual(newState.preferences?.midi.inputDevice, "Test MIDI")
    }

    func testGetPreferencesActionCreation() {
        // When
        let action = getPreferencesAction()

        // Then
        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasGet)
            XCTAssertEqual(request.get.entity, .preferences)
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testUpdatePreferencesActionCreation() {
        // Given
        let preferences = Bloop_Preferences.with {
            $0.audio = Bloop_AudioPreferences.with {
                $0.outputDevice = "Updated Device"
            }
        }

        // When
        let action = updatePreferencesAction(preferences)

        // Then
        if case .sendRequest(let request) = action {
            XCTAssertTrue(request.hasUpdate)
            XCTAssertTrue(request.update.hasPreferences)
            XCTAssertEqual(request.update.preferences.audio.outputDevice, "Updated Device")
        } else {
            XCTFail("Expected sendRequest action")
        }
    }

    func testSetPreferencesWithSwitchMappings() {
        // Given
        let initialState = AppState()
        let mapping = Bloop_SwitchMapping.with {
            $0.pin = 1
            $0.gesture = .press
            $0.action = .togglePlay
        }
        let preferences = Bloop_Preferences.with {
            $0.switch = Bloop_SwitchPreferences.with {
                $0.mappings = [mapping]
            }
        }

        // When
        let action = Action.setPreferences(preferences)
        let newState = rootReducer(state: initialState, action: action)

        // Then
        XCTAssertNotNil(newState.preferences)
        XCTAssertEqual(newState.preferences?.switch.mappings.count, 1)
        XCTAssertEqual(newState.preferences?.switch.mappings.first?.pin, 1)
        XCTAssertEqual(newState.preferences?.switch.mappings.first?.gesture, .press)
        XCTAssertEqual(newState.preferences?.switch.mappings.first?.action, .togglePlay)
    }
}
