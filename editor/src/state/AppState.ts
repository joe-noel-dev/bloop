import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';
import {emptyProject} from '../api/project-helpers';
import {SampleInCache} from '../audio/SampleManager';

export type SaveState = 'idle' | 'saving' | 'saved';

export interface AppState {
  project: Project;
  projectInfo?: DbProject;
  projects: DbProject[];
  playing: boolean;
  playingSongId?: Long;
  playingSectionId?: Long;
  saveState: SaveState;
  sampleStates: Map<Long, SampleInCache>;
}

export const AppStateContext = createContext<AppState>({
  project: emptyProject(),
  projects: [],
  playing: false,
  saveState: 'idle',
  sampleStates: new Map(),
});

export const useAppState = () => useContext(AppStateContext);
