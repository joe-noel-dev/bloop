import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';
import {Sample} from '../../model/sample';

export const useSampleWithId = (sampleId: string) => {
  const {project} = useContext(CoreDataContext);
  return project?.songs.reduce<Sample | undefined>((found, song) => {
    if (found) {
      return found;
    }

    if (song.sample?.id === sampleId) {
      return song.sample;
    }

    return undefined;
  }, undefined);
};
