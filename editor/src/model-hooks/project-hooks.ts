import {useAppState} from '../state/AppState';

export const useProject = () => useAppState().project;
export const useProjectInfo = () => useAppState().projectInfo;
export const useProjects = () => useAppState().projects;
export const useSaveState = () => useAppState().saveState;
