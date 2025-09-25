import {Project} from '../api/bloop';

export type Samples = Map<Long, AudioBuffer>;

export const createSampleManager = (samples: Samples) => {
  const addSample = (id: Long) => {
    console.log('Adding sample', id.toString());
    samples.set(id, new AudioBuffer({length: 1, sampleRate: 44100}));
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

  return {
    setProject,
  };
};

export type SampleManager = ReturnType<typeof createSampleManager>;
