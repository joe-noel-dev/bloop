#pragma once

class InputStream {
public:

  static constexpr auto NO_BEAT = -1;

  void read() {
    if (Serial.available() > 0) {
      const auto bytesRead = Serial.readBytesUntil(';', buffer, bufferLength);
      buffer[bytesRead] = 0;

      if (strncmp("beat:", buffer, 5) == 0 && bytesRead > 5)
        setBeat(atoi(&buffer[5]));
    }

    if (millis() - lastBeatTime > beatTimeout)
      setBeat(NO_BEAT);
  }

  void (*onBeat)(int beat) = nullptr;

private:
  static constexpr auto bufferLength = 128;
  static constexpr auto beatTimeout = 2000;

  void setBeat(int newBeat) {
    if (newBeat != NO_BEAT)
      lastBeatTime = millis();

    beat = newBeat;

    if (onBeat != nullptr)
      (onBeat)(beat);
  }

  char buffer[bufferLength] = { 0 };
  int beat{ NO_BEAT };
  unsigned long lastBeatTime{};
};