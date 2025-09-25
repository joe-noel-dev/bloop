import {PlaybackState, Project} from '../api/bloop';
import {createSampleManager, Samples} from './SampleManager';

export type AudioControllerEvent = {
  state: PlaybackState;
};

export const createAudioController = () => {
  // const audioContext = new AudioContext();
  const samples: Samples = new Map();
  const sampleManager = createSampleManager(samples);

  const setProject = (project: Project) => {
    sampleManager.setProject(project);
  };

  return {
    setProject,
  };
};

export type AudioController = ReturnType<typeof createAudioController>;
