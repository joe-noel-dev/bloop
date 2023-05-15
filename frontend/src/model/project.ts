import {ProjectInfo} from './project-info';
import {Sample} from './sample';
import {Section} from './section';
import {Song} from './song';
import {Selections} from './selections';

export interface Project {
  info: ProjectInfo;
  songs: Song[];
  sections: Section[];
  samples: Sample[];
  selections: Selections;
}

export const projectConstants = {
  MAX_CHANNELS: 8,
};

export function emptyProject(): Project {
  return {
    info: {
      id: '',
      name: '',
      version: '',
      lastSaved: 0,
    },
    songs: [],
    sections: [],
    samples: [],
    selections: {},
  };
}
