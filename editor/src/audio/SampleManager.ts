import {Project} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';
import {setSampleStateAction} from '../dispatcher/action';
import {DispatchFunction} from '../dispatcher/middleware';

export interface SampleInCache {
  state: 'loading' | 'converting' | 'loaded' | 'error';
  buffer?: AudioBuffer;
}

export type Samples = Map<Long, SampleInCache>;

export const createSampleManager = (
  context: AudioContext,
  samples: Samples,
  backend: Backend,
  dispatch: DispatchFunction
) => {
  let project: DbProject | null = null;

  const addSample = async (id: Long) => {
    if (!project) {
      console.error('Project is not set. Cannot load sample.');
      return;
    }

    console.log('Adding sample', id.toString());
    const loadingState = {state: 'loading' as const};
    samples.set(id, loadingState);
    dispatch(setSampleStateAction(id, loadingState));

    const audioFileData = await backend.fetchSample(project, id);
    if (!audioFileData) {
      console.error(`Sample with ID ${id} not found in backend.`);
      const errorState = {state: 'error' as const};
      samples.set(id, errorState);
      dispatch(setSampleStateAction(id, errorState));
      return;
    }

    const convertingState = {state: 'converting' as const};
    samples.set(id, convertingState);
    dispatch(setSampleStateAction(id, convertingState));
    try {
      const audioBuffer = await blobToAudioBuffer(context, audioFileData);
      const loadedState = {state: 'loaded' as const, buffer: audioBuffer};
      samples.set(id, loadedState);
      dispatch(setSampleStateAction(id, loadedState));
      console.log(`Sample ${id} loaded and converted.`);
    } catch (error) {
      console.error(`Error converting audio file for sample ${id}:`, error);
      const errorState = {state: 'error' as const};
      samples.set(id, errorState);
      dispatch(setSampleStateAction(id, errorState));
      return;
    }
  };

  const syncSamples = (project: Project) => {
    const requiredIds = project.songs.reduce((ids, song) => {
      if (song.sample) {
        ids.add(song.sample.id);
      }
      return ids;
    }, new Set<Long>());

    const existingIds = new Set(samples.keys());

    for (const id of existingIds) {
      if (!requiredIds.has(id)) {
        console.log('Removing sample', id.toString());
        samples.delete(id);
      }
    }

    for (const id of requiredIds) {
      if (!samples.has(id)) {
        addSample(id);
      }
    }
  };

  const setProject = (project: Project) => {
    syncSamples(project);
  };

  const setProjectInfo = (projectInfo: DbProject) => {
    project = projectInfo;
  };

  return {
    setProject,
    setProjectInfo,
  };
};

export type SampleManager = ReturnType<typeof createSampleManager>;

export async function blobToAudioBuffer(
  context: AudioContext,
  blob: Blob
): Promise<AudioBuffer> {
  const arrayBuf = await blob.arrayBuffer();
  return await context.decodeAudioData(arrayBuf);
}
