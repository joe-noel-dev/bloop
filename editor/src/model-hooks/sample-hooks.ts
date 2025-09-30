import {Sample} from '../api/bloop';
import {ID} from '../api/helpers';
import {useProject} from './project-hooks';
import {useAppState} from '../state/AppState';

export const useSampleWithId = (sampleId: ID) => {
  const project = useProject();
  return project?.songs.reduce<Sample | undefined>((found, song) => {
    if (found) {
      return found;
    }

    if (song.sample?.id.equals(sampleId)) {
      return song.sample;
    }

    return undefined;
  }, undefined);
};

export const useSampleState = (sampleId: ID) => {
  const appState = useAppState();
  return appState.sampleStates.get(sampleId)?.state ?? null;
};
