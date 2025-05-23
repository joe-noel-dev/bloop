import {createContext, useContext} from 'react';
import {Project} from '../api/bloop';
import {DbProject} from '../backend/Backend';

interface CoreData {
  project?: Project;
  projectInfo: DbProject | null;
  projects: DbProject[];
}

export const CoreDataContext = createContext<CoreData>({
  projects: [],
  projectInfo: null,
});

export const useCoreData = () => useContext(CoreDataContext);
