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

export const addSection = (
  project: Project,
  songId: ID,
  section?: Partial<Section>
) => {
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

  if (section) {
    Object.assign(newSection, section);
  }

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

export const moveSection = (
  project: Project,
  songId: ID,
  fromIndex: number,
  toIndex: number
) => {
  const song = project.songs.find((song) => song.id.equals(songId));
  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  if (fromIndex < 0 || fromIndex >= song.sections.length) {
    console.error(
      `Invalid fromIndex: ${fromIndex}. Must be between 0 and ${
        song.sections.length - 1
      }`
    );
    return;
  }

  if (toIndex < 0 || toIndex >= song.sections.length) {
    console.error(
      `Invalid toIndex: ${toIndex}. Must be between 0 and ${
        song.sections.length - 1
      }`
    );
    return;
  }

  const startPositions = song.sections.map((s) => s.start);
  const newSong = {...song};
  const [movedSection] = newSong.sections.splice(fromIndex, 1);
  newSong.sections.splice(toIndex, 0, movedSection);

  newSong.sections.forEach((section, index) => {
    section.start = startPositions[index];
  });

  updateSong(project, newSong);
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

export const removeSection = (project: Project, songId: ID, sectionId: ID) => {
  const song = project.songs.find((song) => song.id.equals(songId));

  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  if (song.sections.length <= 1) {
    console.error(`Cannot remove the last section from song ${songId}`);
    return;
  }

  song.sections = song.sections.filter(
    (section) => !section.id.equals(sectionId)
  );

  if (project.selections && project.selections.section.equals(sectionId)) {
    project.selections.section = song.sections[0]?.id;
  }
};

export const removeSong = (project: Project, songId: ID) => {
  if (project.songs.length <= 1) {
    console.error(`Cannot remove the last song from the project`);
    return;
  }

  project.songs = project.songs.filter((song) => !song.id.equals(songId));

  if (project.selections && project.selections.song.equals(songId)) {
    project.selections.song = project.songs[0]?.id;
    project.selections.section = project.songs[0]?.sections[0]?.id;
  }
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

export const splitSection = (project: Project, songId: ID, sectionId: ID) => {
  const song = project.songs.find((song) => song.id.equals(songId));
  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  const sectionIndex = song.sections.findIndex((section) =>
    section.id.equals(sectionId)
  );
  if (sectionIndex === -1) {
    console.error(`Section with ID ${sectionId} not found`);
    return;
  }

  const originalSection = song.sections[sectionIndex];
  const nextSection = song.sections[sectionIndex + 1];

  // Calculate the duration of the original section
  const sectionEnd = nextSection
    ? nextSection.start
    : originalSection.start + 16.0; // Default to 16 beats if it's the last section
  const sectionDuration = sectionEnd - originalSection.start;
  const splitPoint = originalSection.start + sectionDuration / 2;

  // Create a new section that starts at the split point
  const newSection: Section = {
    id: randomId(),
    name: `${originalSection.name}`,
    start: splitPoint,
    loop: originalSection.loop,
    metronome: originalSection.metronome,
  };

  // Insert the new section after the original section
  song.sections.splice(sectionIndex + 1, 0, newSection);
};

export const emptyProject = (): Project => {
  const song = defaultSong();
  return {
    songs: [song],
    selections: {
      song: song.id,
      section: song.sections[0]?.id,
    },
  };
};
