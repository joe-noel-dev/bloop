import {createContext, useContext} from 'react';
import {
  PlaybackState,
  Progress,
  Project,
  ProjectInfo,
  WaveformData,
} from '../api/bloop';
import {ID} from '../api/helpers';

interface CoreData {
  project?: Project;
  playbackState?: PlaybackState;
  projects: ProjectInfo[];
  waveforms: Map<ID, WaveformData>;
  progress: Progress;
}

export const CoreDataContext = createContext<CoreData>({
  projects: [],
  waveforms: new Map(),
  progress: {
    songProgress: 0,
    sectionProgress: 0,
    sectionBeat: 0,
  },
});

export const useCoreData = () => useContext(CoreDataContext);
