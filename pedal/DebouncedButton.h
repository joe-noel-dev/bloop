#pragma once


class DebouncedButton {
public:
  DebouncedButton(int pin, int buttonId)
    : buttonPin{ pin },
      buttonId{ buttonId } {
    pinMode(buttonPin, INPUT_PULLUP);
  }

  void read() {
    const auto reading = digitalRead(buttonPin);

    if (reading != lastState) {
      lastDebounceTime = millis();
    }

    if ((millis() - lastDebounceTime) > debounceDelay) {

      if (currentState != reading) {
        currentState = reading;

        if (currentState == LOW && onPress != nullptr) {
          (onPress)(buttonId);
        }

        if (currentState == HIGH && onRelease != nullptr) {
          (onRelease)(buttonId);
        }
      }
    }

    lastState = reading;
  }

  bool isPressed() const {
    return currentState == LOW;
  }

  void (*onPress)(int index) = nullptr;

  void (*onRelease)(int index) = nullptr;

private:
  static constexpr auto debounceDelay{ 5 };

  int buttonPin;
  int buttonId;
  int lastState{ HIGH };
  int currentState{ HIGH };
  unsigned long lastDebounceTime{};
};