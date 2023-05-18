import {Entity} from './entity';
import {Tempo, beatFrequency} from './tempo';

export interface Sample extends Entity {
  id: string;
  name: string;
  tempo: Tempo;
  sampleRate: number;
  sampleCount: number;
  channelCount: number;
}

export const getSampleBeatLength = (sample: Sample): number =>
  (sample.sampleCount * beatFrequency(sample.tempo)) / sample.sampleRate;
