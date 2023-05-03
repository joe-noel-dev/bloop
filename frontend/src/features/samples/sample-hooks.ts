import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useSampleWithId = (sampleId: string) => {
  const {project} = useContext(CoreDataContext);
  return project?.samples.find((sample) => sample.id === sampleId);
};
