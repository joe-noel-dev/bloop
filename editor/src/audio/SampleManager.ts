import {Project} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';

export interface SampleInCache {
  state: 'loading' | 'converting' | 'loaded' | 'error';
  buffer?: AudioBuffer;
}

export type Samples = Map<Long, SampleInCache>;

export const createSampleManager = (
  context: AudioContext,
  samples: Samples,
  backend: Backend
) => {
  let project: DbProject | null = null;

  const addSample = async (id: Long) => {
    if (!project) {
      console.error('Project is not set. Cannot load sample.');
      return;
    }

    console.log('Adding sample', id.toString());
    samples.set(id, {state: 'loading'});

    const audioFileData = await backend.fetchSample(project, id);
    if (!audioFileData) {
      console.error(`Sample with ID ${id} not found in backend.`);
      samples.set(id, {state: 'error'});
      return;
    }

    samples.set(id, {state: 'converting'});
    try {
      const audioBuffer = await blobToAudioBuffer(context, audioFileData);
      samples.set(id, {state: 'loaded', buffer: audioBuffer});
      console.log(`Sample ${id} loaded and converted.`);
    } catch (error) {
      console.error(`Error converting audio file for sample ${id}:`, error);
      samples.set(id, {state: 'error'});
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
