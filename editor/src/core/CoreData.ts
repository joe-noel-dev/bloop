import {createContext, useContext} from 'react';
import {PlaybackState} from '../model/playback-state';
import {Project} from '../model/project';
import {ProjectInfo} from '../model/project-info';
import {WaveformData} from '../model/waveform';
import {Progress} from '../model/progress';

interface CoreData {
  project?: Project;
  playbackState?: PlaybackState;
  projects: ProjectInfo[];
  waveforms: Map<string, WaveformData>;
}

export const CoreDataContext = createContext<CoreData>({
  projects: [],
  waveforms: new Map(),
});

export const useCoreData = () => useContext(CoreDataContext);

export const ProgressContext = createContext<Progress>({
  songProgress: 0,
  sectionProgress: 0,
  sectionBeat: 0,
});

export const useProgress = () => useContext(ProgressContext);
