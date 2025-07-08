import '@testing-library/jest-dom';

// Mock Web Audio API for tests
global.AudioContext = class MockAudioContext {
  decodeAudioData() {
    return Promise.resolve({
      sampleRate: 44100,
      numberOfChannels: 2,
      length: 1000,
    });
  }
} as any;

// Mock File API
global.File = class MockFile {
  public name: string;

  constructor(chunks: any[], name: string) {
    this.name = name;
  }

  arrayBuffer() {
    return Promise.resolve(new ArrayBuffer(0));
  }
} as any;
