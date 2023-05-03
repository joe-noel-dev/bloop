import {Entity} from './entity';
import {Tempo} from './tempo';

export interface Sample extends Entity {
  id: string;
  name: string;
  tempo: Tempo;
  sampleRate: number;
  sampleCount: number;
  channelCount: number;
}

export function beatLength(sample: Sample): number {
  return (sample.sampleCount * sample.tempo.bpm) / (60.0 * sample.sampleRate);
}
