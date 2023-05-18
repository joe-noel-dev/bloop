import {Entity} from './entity';
import {Metronome} from './metronome';
import {Sample} from './sample';
import {Section} from './section';
import {Tempo} from './tempo';

export interface Song extends Entity {
  name: string;
  tempo: Tempo;
  metronome: Metronome;
  sections: Section[];
  sample?: Sample;
}
