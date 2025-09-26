import {PlaybackState, Project} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';
import {createSampleManager, Samples} from './SampleManager';

export type AudioControllerEvent = {
  state: PlaybackState;
};

export const createAudioController = (backend: Backend) => {
  // const audioContext = new AudioContext();
  const samples: Samples = new Map();
  const sampleManager = createSampleManager(samples, backend);

  const setProject = (project: Project) => {
    sampleManager.setProject(project);
  };

  const setProjectInfo = (projectInfo: DbProject) => {
    sampleManager.setProjectInfo(projectInfo);
  };

  return {
    setProject,
    setProjectInfo,
  };
};

export type AudioController = ReturnType<typeof createAudioController>;
