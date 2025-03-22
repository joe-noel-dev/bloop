import Long from 'long';
import {Project, Sample, Song, Tempo} from './bloop';

export type ID = Long;
export const INVALID_ID = Long.fromNumber(0);

export const projectConstants = {
  MAX_CHANNELS: 8,
};

export const randomId = (): Long => {
  const low = Math.floor(Math.random() * 0x100000000); // 32-bit unsigned
  const high = Math.floor(Math.random() * 0x100000000); // 32-bit unsigned
  return Long.fromBits(low, high, true); // `true` = unsigned
};

export function emptyProject(): Project {
  return {
    info: {
      id: randomId(),
      name: '',
      version: '',
      lastSaved: Long.fromNumber(0),
    },
    songs: [],
    selections: {
      song: Long.fromNumber(0),
      section: Long.fromNumber(0),
    },
  };
}

export const getSampleBeatLength = (sample: Sample): number =>
  sample.tempo
    ? (sample.sampleCount.toNumber() * beatFrequency(sample.tempo)) /
      sample.sampleRate
    : 0;

const getSongBeatLength = (song: Song) =>
  song.sample ? getSampleBeatLength(song.sample) : 0.0;

export const getSectionBeatLength = (song: Song, sectionId: Long) => {
  const sections = song.sections;

  const index = sections.findIndex((section) => section.id === sectionId);
  if (index < 0) {
    return 0;
  }

  const section = sections.at(index);
  const nextSection = sections.at(index + 1);
  const start = section?.start ?? 0.0;

  const end = nextSection ? nextSection.start : getSongBeatLength(song);

  if (start <= end) {
    return end - start;
  }

  return 0;
};

export const updateSectionBeatLength = (
  song: Song,
  sectionId: Long,
  duration: number
) => {
  if (duration < 0) {
    return;
  }

  const existingDuration = getSectionBeatLength(song, sectionId);
  const durationDelta = duration - existingDuration;

  const sectionIndex = song.sections.findIndex(
    (section) => section.id === sectionId
  );
  if (sectionIndex < 0) {
    return;
  }

  for (let i = sectionIndex + 1; i < song.sections.length; i++) {
    const section = song.sections[i];
    section.start += durationDelta;
  }
};

export const beatFrequency = (tempo: Tempo) => tempo.bpm / 60.0;
