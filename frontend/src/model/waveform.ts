export interface WaveformPeaks {
  properties: {
    algorithm: string;
    channel: number;
    length: number;
  };
  values: number[];
}

export interface WaveformData {
  sampleRate: number;
  peaks: WaveformPeaks[];
}

export interface Waveform {
  sampleId: string;
  waveformData: WaveformData;
}
