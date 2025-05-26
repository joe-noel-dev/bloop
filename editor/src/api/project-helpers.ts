import {Project, Section, Song} from './bloop';
import {ID, randomId} from './helpers';

const defaultSection = (): Section => {
  return {
    id: randomId(),
    name: 'Section',
    start: 0,
    loop: false,
    metronome: false,
  };
};

const defaultSong = (): Song => {
  return {
    id: randomId(),
    name: 'Song',
    tempo: {
      bpm: 120,
    },
    sections: [defaultSection()],
  };
};

export const addSong = (project: Project) => {
  const newSong = defaultSong();
  project.songs.push(newSong);
  project.selections = {
    song: newSong.id,
    section: newSong.sections[0].id,
  };
};

export const addSection = (project: Project, songId: ID) => {
  const song = project.songs.find((song) => song.id.equals(songId));
  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return project;
  }

  const lastSection = song.sections[song.sections.length - 1];
  const lastSectionStart = lastSection ? lastSection.start : 0;

  const newSection = defaultSection();
  const DEFAULT_SECTION_LENGTH = 16.0;
  newSection.start = lastSectionStart + DEFAULT_SECTION_LENGTH;

  song.sections.push(newSection);
};

export const selectSong = (project: Project, songId: ID) => {
  const song = project.songs.find((song) => song.id.equals(songId));

  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return project;
  }

  project.selections = {
    song: songId,
    section: song.sections[0]?.id || undefined,
  };
};

export const moveSong = (
  project: Project,
  fromIndex: number,
  toIndex: number
) => {
  if (fromIndex < 0 || fromIndex >= project.songs.length) {
    console.error(
      `Invalid fromIndex: ${fromIndex}. Must be between 0 and ${
        project.songs.length - 1
      }`
    );
    return;
  }

  if (toIndex < 0 || toIndex >= project.songs.length) {
    console.error(
      `Invalid toIndex: ${toIndex}. Must be between 0 and ${
        project.songs.length - 1
      }`
    );
    return;
  }

  const newSongs = [...project.songs];
  newSongs.splice(toIndex, 0, newSongs.splice(fromIndex, 1)[0]);
  project.songs = newSongs;
};

export const updateSection = (
  project: Project,
  songId: ID,
  newSection: Section
) => {
  const song = project.songs.find((song) => song.id.equals(songId));
  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  const section = song.sections.find((section) =>
    section.id.equals(newSection.id)
  );
  if (!section) {
    console.error(
      `Section with ID ${newSection.id} not found in song ${songId}`
    );
    return;
  }

  Object.assign(section, newSection);
};

export const updateSong = (project: Project, newSong: Song) => {
  const song = project.songs.find((song) => song.id.equals(newSong.id));
  if (!song) {
    console.error(`Song with ID ${newSong.id} not found`);
    return;
  }

  Object.assign(song, newSong);
};
