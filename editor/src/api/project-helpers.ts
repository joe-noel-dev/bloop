import {Project, Section, Song} from './bloop';
import {randomId} from './helpers';

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

export const addSong = (project: Project): Project => {
  const newProject = {...project};
  const newSong = defaultSong();
  newProject.songs.push(newSong);
  newProject.selections = {
    song: newSong.id,
    section: newSong.sections[0].id,
  };
  return newProject;
};
