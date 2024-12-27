import {Entity} from './entity';
import {Sample, getSampleBeatLength} from './sample';
import {Section} from './section';
import {Tempo} from './tempo';

export interface Song extends Entity {
  name: string;
  tempo: Tempo;
  sections: Section[];
  sample?: Sample;
}

const getSongBeatLength = (song: Song) =>
  song.sample ? getSampleBeatLength(song.sample) : 0.0;

export const getSectionBeatLength = (song: Song, sectionId: string) => {
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
  sectionId: string,
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
