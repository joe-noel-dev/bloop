import {useCoreData} from '../core/CoreData';

export const useProject = () => useCoreData().project;

export const useProjects = () => useCoreData().projects;
