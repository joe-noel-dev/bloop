import {describe, expect, it, vi} from 'vitest';
import Long from 'long';
import {emptyProject} from '../api/project-helpers';
import {DbProject} from './Backend';
import {removeUnusedSamples} from './BackendMiddleware';

const projectInfo = (samples: string[]): DbProject => ({
  collectionId: 'projects',
  collectionName: 'projects',
  created: new Date(),
  id: 'project-id',
  name: 'Project',
  project: 'project.bin',
  samples,
  userId: 'user-id',
});

describe('removeUnusedSamples', () => {
  it('keeps samples referenced by the project, even when their Long instances differ', async () => {
    const usedSampleId = Long.fromString('123456789');
    const unusedSampleId = Long.fromString('987654321');
    const project = emptyProject();
    project.songs = [
      {
        id: Long.fromNumber(1),
        name: 'Song',
        tempo: {bpm: 120},
        sections: [],
        sample: {
          id: usedSampleId,
          name: 'used.wav',
          tempo: {bpm: 120},
          sampleRate: 44100,
          sampleCount: Long.fromNumber(1),
          channelCount: 2,
        },
      },
    ];
    const removeSample = vi.fn();
    const backend = {
      fetchProjectInfo: vi
        .fn()
        .mockResolvedValue(
          projectInfo([`${usedSampleId}_used.wav`, `${unusedSampleId}_old.wav`])
        ),
      getIdFromSampleFileName: (fileName: string) =>
        Long.fromString(fileName.split('_')[0]),
      removeSample,
    };

    await removeUnusedSamples(project, projectInfo([]), backend);

    expect(backend.fetchProjectInfo).toHaveBeenCalledWith('project-id');
    expect(removeSample).toHaveBeenCalledTimes(1);
    expect(removeSample).toHaveBeenCalledWith('project-id', unusedSampleId);
  });
});
