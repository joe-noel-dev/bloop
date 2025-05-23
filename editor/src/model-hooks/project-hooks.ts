import {useCoreData} from '../core/CoreData';

export const useProject = () => useCoreData().project;
export const useProjectInfo = () => useCoreData().projectInfo;
export const useProjects = () => useCoreData().projects;
