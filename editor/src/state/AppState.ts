import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';

export interface AppState {
  project?: Project;
  projectInfo: DbProject | null;
  projects: DbProject[];
}

export const AppStateContext = createContext<AppState>({
  projects: [],
  projectInfo: null,
});

export const useAppState = () => useContext(AppStateContext);
