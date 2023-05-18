export interface Tempo {
  bpm: number;
}

export const beatFrequency = (tempo: Tempo) => tempo.bpm / 60.0;
