import {Project} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';
import {setSampleStateAction} from '../dispatcher/action';
import {DispatchFunction} from '../dispatcher/middleware';

export type SampleState = 'loading' | 'converting' | 'loaded' | 'error';

export interface SampleInCache {
  state: SampleState;
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

  const setSampleState = (id: Long, state: SampleState, buffer?: AudioBuffer) => {
    const sampleInCache: SampleInCache = { state, buffer };
    samples.set(id, sampleInCache);
    dispatch(setSampleStateAction(id, state));
  };

  const addSample = async (id: Long) => {
    if (!project) {
      console.error('Project is not set. Cannot load sample.');
      return;
    }

    console.log('Adding sample', id.toString());
    setSampleState(id, 'loading');

    const audioFileData = await backend.fetchSample(project, id);
    if (!audioFileData) {
      console.error(`Sample with ID ${id} not found in backend.`);
      setSampleState(id, 'error');
      return;
    }

    setSampleState(id, 'converting');
    try {
      const audioBuffer = await blobToAudioBuffer(context, audioFileData);
      setSampleState(id, 'loaded', audioBuffer);
      console.log(`Sample ${id} loaded and converted.`);
    } catch (error) {
      console.error(`Error converting audio file for sample ${id}:`, error);
      setSampleState(id, 'error');
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
