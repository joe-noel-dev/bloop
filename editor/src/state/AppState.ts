import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';
import {emptyProject} from '../api/project-helpers';

export interface AppState {
  project: Project;
  projectInfo?: DbProject;
  projects: DbProject[];
}

export const AppStateContext = createContext<AppState>({
  project: emptyProject(),
  projects: [],
});

export const useAppState = () => useContext(AppStateContext);
