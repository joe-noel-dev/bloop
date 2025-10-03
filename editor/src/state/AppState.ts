import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';
import {emptyProject} from '../api/project-helpers';
import {SampleInCache} from '../audio/SampleManager';
import {ThemeState, createThemeState} from './ThemeState';

export type SaveState = 'idle' | 'saving' | 'saved';

export interface AppState {
  project: Project;
  projectInfo: DbProject | null;
  projects: DbProject[];
  playing: boolean;
  playingSongId?: Long;
  playingSectionId?: Long;
  saveState: SaveState;
  sampleStates: Map<Long, SampleInCache>;
  theme: ThemeState;
}

export const emptyAppState = (): AppState => ({
  project: emptyProject(),
  projectInfo: null,
  projects: [],
  playing: false,
  saveState: 'idle',
  sampleStates: new Map(),
  theme: createThemeState(),
});

export const AppStateContext = createContext<AppState>(emptyAppState());

export const useAppState = () => useContext(AppStateContext);
