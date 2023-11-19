import {createContext} from 'react';
import {PlaybackState} from '../model/playback-state';
import {Progress} from '../model/progress';
import {Project} from '../model/project';
import {ProjectInfo} from '../model/project-info';
import {WaveformData} from '../model/waveform';

interface CoreData {
  project?: Project;
  playbackState?: PlaybackState;
  progress?: Progress;
  projects: ProjectInfo[];
  waveforms: Map<string, WaveformData>;
}

export const CoreDataContext = createContext<CoreData>({
  projects: [],
  waveforms: new Map(),
});
