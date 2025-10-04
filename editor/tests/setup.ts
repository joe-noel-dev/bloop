import '@testing-library/jest-dom';
import {vi} from 'vitest';

// Make vi globally available
(globalThis as any).vi = vi;

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

  constructor(_chunks: any[], name: string) {
    this.name = name;
  }

  arrayBuffer() {
    return Promise.resolve(new ArrayBuffer(0));
  }
} as any;
