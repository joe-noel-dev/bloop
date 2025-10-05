import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';
import {emptyProject} from '../api/project-helpers';
import {SampleInCache} from '../audio/SampleManager';
import {ThemeState, createThemeState} from './ThemeState';
import {PlaybackState} from '../audio/AudioController';

export type SaveState = 'idle' | 'saving' | 'saved';

export interface AppState {
  project: Project;
  projectInfo: DbProject | null;
  projects: DbProject[];
  playbackState: PlaybackState | null;
  saveState: SaveState;
  sampleStates: Map<Long, SampleInCache>;
  theme: ThemeState;
}

export const emptyAppState = (): AppState => ({
  project: emptyProject(),
  projectInfo: null,
  projects: [],
  playbackState: null,
  saveState: 'idle',
  sampleStates: new Map(),
  theme: createThemeState(),
});

export const AppStateContext = createContext<AppState>(emptyAppState());

export const useAppState = () => useContext(AppStateContext);
