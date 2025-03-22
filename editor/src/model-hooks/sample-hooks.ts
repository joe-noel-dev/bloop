import {Sample} from '../api/bloop';
import {ID} from '../api/helpers';
import {useProject} from './project-hooks';

export const useSampleWithId = (sampleId: ID) => {
  const project = useProject();
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
