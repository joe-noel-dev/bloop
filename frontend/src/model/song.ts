import {Entity} from './entity';
import {Metronome} from './metronome';
import {Tempo} from './tempo';

export interface Song extends Entity {
  name: string;
  tempo: Tempo;
  metronome: Metronome;
  sectionIds: string[];
  sampleId: string;
}
