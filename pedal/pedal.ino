#include "DebouncedButton.h"
#include "InputStream.h"

static constexpr int LED_PINS[] = { 2, 3, 4, 5 };
static constexpr auto LED_COUNT = sizeof(LED_PINS) / sizeof(LED_PINS[0]);
static constexpr auto BAUD_RATE = 9600;

static DebouncedButton buttons[] = {
  DebouncedButton(6, 0),
  DebouncedButton(7, 1),
  DebouncedButton(8, 2),
};

static InputStream inputStream;

void onButtonPress(int buttonIndex) {
  Serial.print("press:");
  Serial.print(buttonIndex);
  Serial.print(";");
}

void onButtonRelease(int buttonIndex) {
  Serial.print("release:");
  Serial.print(buttonIndex);
  Serial.print(";");
}

void onBeat(int beat) {
  for (int ledIndex = 0; ledIndex < LED_COUNT; ++ledIndex) {
    const auto ledState =
      beat == InputStream::NO_BEAT ? LOW : beat % LED_COUNT == ledIndex;
    digitalWrite(LED_PINS[ledIndex], ledState);
  }
}

void setUpLeds() {
  for (auto &&pin : LED_PINS) {
    pinMode(pin, OUTPUT);
  }
}

void setUpButtons() {
  for (auto &&button : buttons) {
    button.onPress = onButtonPress;
    button.onRelease = onButtonRelease;
  }
}

void setUpInputStream() {
  inputStream.onBeat = onBeat;
}

void setup() {
  Serial.begin(BAUD_RATE);
  setUpLeds();
  setUpButtons();
  setUpInputStream();
}

void loop() {
  for (auto &&button : buttons) {
    button.read();
  }

  inputStream.read();
}
