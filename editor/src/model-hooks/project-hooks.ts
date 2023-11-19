import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useProject = () => useContext(CoreDataContext).project;
