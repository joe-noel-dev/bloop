import {Project, Section, Song} from '../api/bloop';
import {Backend, DbProject} from '../backend/Backend';
import {createSampleManager, Samples} from './SampleManager';
import {ID} from '../api/helpers';
import Long from 'long';
import {DispatchFunction} from '../dispatcher/middleware';

export interface PlaybackState {
  songId: ID | null;
  sectionId: ID | null;
}

export type PlaybackStateChangeCallback = (
  playbackState: PlaybackState | null
) => void;

export const createAudioController = (backend: Backend) => {
  const audioContext = new AudioContext();
  const samples: Samples = new Map();
  let dispatch: DispatchFunction | null = null;
  let playbackState: PlaybackState | null = null;

  const sampleManager = createSampleManager(
    audioContext,
    samples,
    backend,
    (action) => {
      if (dispatch) {
        dispatch(action);
      }
    }
  );
  let project: Project | null = null;
  let projectInfo: DbProject | null = null;
  let playbackStateChangeCallback: PlaybackStateChangeCallback | null = null;

  let bufferNodes: AudioBufferSourceNode[] = [];
  interface SchedulePoint {
    start: number;
    end?: number;
    songId: Long;
    sectionId: Long;
  }
  let playbackSchedule: SchedulePoint[] = [];
  let callbackId: number | null = null;

  const setProject = (newProject: Project) => {
    project = newProject;
    sampleManager.setProject(project);
  };

  const setProjectInfo = (newProjectInfo: DbProject) => {
    projectInfo = newProjectInfo;
    sampleManager.setProjectInfo(projectInfo);
  };

  const setPlaybackState = (newState: PlaybackState | null) => {
    playbackState = newState;
    if (playbackStateChangeCallback) {
      playbackStateChangeCallback(playbackState);
    }
  };

  const scheduleSection = (
    sample: AudioBuffer,
    song: Song,
    section: Section,
    nextSection: Section | undefined,
    startTime: number
  ): number | undefined => {
    const bufferNode = audioContext.createBufferSource();
    bufferNodes.push(bufferNode);

    bufferNode.connect(audioContext.destination);
    bufferNode.buffer = sample;
    bufferNode.loop = section.loop;

    const tempo = song.tempo?.bpm ?? 120;
    const beatFrequency = tempo / 60.0;
    const beatInterval = 1.0 / beatFrequency;

    const startPosInSample = section.start * beatInterval;
    const endPosInSample = nextSection
      ? nextSection.start * beatInterval
      : undefined;
    const duration =
      !section.loop && endPosInSample
        ? endPosInSample - startPosInSample
        : undefined;

    if (section.loop) {
      bufferNode.loopStart = startPosInSample;
    }

    if (section.loop && endPosInSample) {
      bufferNode.loopEnd = endPosInSample;
    }

    bufferNode.start(startTime, startPosInSample, duration);

    playbackSchedule.push({
      start: startTime,
      end: duration ? startTime + duration : undefined,
      songId: song.id,
      sectionId: section.id,
    });

    return duration;
  };

  const play = (songId: Long, sectionId: Long) => {
    if (!project) {
      console.error('No project loaded. Cannot play.');
      return;
    }

    const song = project.songs.find((s) => s.id.equals(songId));
    if (!song) {
      console.error(`Song with ID ${songId} not found. Cannot play.`);
      return;
    }

    if (!song.sample) {
      console.error(`Song with ID ${songId} has no sample. Cannot play.`);
      return;
    }

    if (audioContext.state === 'suspended') {
      audioContext.resume();
    }

    const sampleInCache = samples.get(song.sample.id);
    if (
      !sampleInCache ||
      sampleInCache.state !== 'loaded' ||
      !sampleInCache.buffer
    ) {
      console.error(
        `Sample with ID ${song.sample.id} not loaded. Cannot play.`
      );
      return;
    }

    stop();

    const startIndex = song.sections.findIndex((sec) =>
      sec.id.equals(sectionId)
    );

    const lookaheadS = 0.05;
    let startTime = audioContext.currentTime + lookaheadS;

    for (
      let sectionIndex = startIndex;
      0 <= sectionIndex && sectionIndex < song.sections.length;
      ++sectionIndex
    ) {
      const section = song.sections.at(sectionIndex);
      const nextSection = song.sections.at(sectionIndex + 1);

      if (!section) {
        console.error(
          `Section at index ${sectionIndex} not found. Stopping playback loop.`
        );
        break;
      }

      const duration = scheduleSection(
        sampleInCache.buffer,
        song,
        section,
        nextSection,
        startTime
      );

      if (duration) {
        startTime += duration;
      }
    }

    const notificationIntervalMs = 15;

    callbackId = window.setInterval(() => {
      const currentTime = audioContext.currentTime;
      const current = playbackSchedule.find((point) => {
        if (point.start > currentTime) {
          return false;
        }

        if (point.end && point.end <= currentTime) {
          return false;
        }

        return true;
      });

      setPlaybackState(
        current ? {songId: current.songId, sectionId: current.sectionId} : null
      );
    }, notificationIntervalMs);
  };

  const stop = () => {
    bufferNodes.forEach((node) => {
      node.stop();
      node.disconnect();
    });

    bufferNodes = [];
    playbackSchedule = [];

    if (callbackId) {
      window.clearInterval(callbackId);
    }

    setPlaybackState(null);
  };

  const setPlaybackStateChangeCallback = (
    callback: PlaybackStateChangeCallback
  ) => {
    playbackStateChangeCallback = callback;
  };

  const setDispatch = (dispatchFunction: DispatchFunction) => {
    dispatch = dispatchFunction;
  };

  const getSampleState = (sampleId: Long) => {
    const sampleInCache = samples.get(sampleId);
    return sampleInCache?.state ?? null;
  };

  return {
    setProject,
    setProjectInfo,
    play,
    stop,
    setPlaybackStateChangeCallback,
    setDispatch,
    getSampleState,
  };
};

export type AudioController = ReturnType<typeof createAudioController>;
