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
