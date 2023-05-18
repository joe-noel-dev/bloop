import {ProjectInfo} from './project-info';
import {Song} from './song';
import {Selections} from './selections';

export interface Project {
  info: ProjectInfo;
  songs: Song[];
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
    selections: {},
  };
}
