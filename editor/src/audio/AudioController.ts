import {PlaybackState, Project} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';
import {createSampleManager, Samples} from './SampleManager';

export type AudioControllerEvent = {
  state: PlaybackState;
};

export const createAudioController = (backend: Backend) => {
  const audioContext = new AudioContext();
  const samples: Samples = new Map();
  const sampleManager = createSampleManager(audioContext, samples, backend);
  let project: Project | null = null;
  let projectInfo: DbProject | null = null;

  let bufferNode: AudioBufferSourceNode | null = null;

  const setProject = (newProject: Project) => {
    project = newProject;
    sampleManager.setProject(project);
  };

  const setProjectInfo = (newProjectInfo: DbProject) => {
    projectInfo = newProjectInfo;
    sampleManager.setProjectInfo(projectInfo);
  };

  const play = (songId: Long, sectionId: Long, loop: boolean) => {
    if (!project) {
      console.error('No project loaded. Cannot play.');
      return;
    }

    const song = project.songs.find((s) => s.id.equals(songId));
    if (!song) {
      console.error(`Song with ID ${songId} not found. Cannot play.`);
      return;
    }

    const sectionIndex = song.sections.findIndex((sec) =>
      sec.id.equals(sectionId)
    );
    if (sectionIndex === -1) {
      console.error(
        `Section with ID ${sectionId} not found in song ${songId}. Cannot play.`
      );
      return;
    }

    const section = song.sections[sectionIndex];
    if (!section) {
      console.error(`Section with ID ${sectionId} not found. Cannot play.`);
      return;
    }

    const nextSection =
      sectionIndex + 1 < song.sections.length
        ? song.sections[sectionIndex + 1]
        : null;

    if (!song.sample) {
      console.error(`Song with ID ${songId} has no sample. Cannot play.`);
      return;
    }

    const sampleInCache = samples.get(song.sample.id);
    if (!sampleInCache || sampleInCache.state !== 'loaded') {
      console.error(
        `Sample with ID ${song.sample.id} not loaded. Cannot play.`
      );
      return;
    }

    if (!sampleInCache.buffer) {
      console.error(`Sample with ID ${song.sample.id} has no audio buffer.`);
      return;
    }

    if (audioContext.state === 'suspended') {
      audioContext.resume();
    }

    if (bufferNode) {
      stop();
    }

    bufferNode = audioContext.createBufferSource();
    bufferNode.connect(audioContext.destination);
    bufferNode.buffer = sampleInCache.buffer;
    bufferNode.loop = loop;

    const tempo = song.tempo?.bpm ?? 120;
    const beatFrequency = tempo / 60.0;
    const beatInterval = 1.0 / beatFrequency;

    const start = section.start * beatInterval;
    const end = nextSection ? nextSection.start * beatInterval : undefined;

    if (loop) {
      bufferNode.loopStart = start;
    }

    if (loop && end) {
      bufferNode.loopEnd = end;
    }

    bufferNode.connect(audioContext.destination);
    bufferNode.start(0, start, !loop && end ? end - start : undefined);
  };

  const stop = () => {
    if (bufferNode) {
      bufferNode.stop();
      bufferNode.disconnect();
      bufferNode = null;
    }
  };

  return {
    setProject,
    setProjectInfo,
    play,
    stop,
  };
};

export type AudioController = ReturnType<typeof createAudioController>;
