import {Entity} from './entity';
import {Metronome} from './metronome';
import {Section} from './section';
import {Tempo} from './tempo';

export interface Song extends Entity {
  name: string;
  tempo: Tempo;
  metronome: Metronome;
  sections: Section[];
  sampleId: string;
}
