import {Sample} from '../model/sample';
import {useProject} from './project-hooks';

export const useSampleWithId = (sampleId: string) => {
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
