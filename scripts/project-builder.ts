import process from 'process';
import fs from 'fs';
import {emptyProject} from '../frontend/src/model/project';
import {randomUUID} from 'crypto';
import {Song} from '../frontend/src/model/song';
import {Section} from '../frontend/src/model/section';

const convertSong = (songSpec: any): Song => {
  const song = {
    id: randomUUID(),
    name: songSpec.name ?? 'Song',
    tempo: {bpm: songSpec.tempo ?? 120},
    sections: songSpec.sections ? songSpec.sections.map(convertSection) : [],
  };

  song.sections.sort((a: Section, b: Section) => a.start - b.start);

  return song;
};

const convertSection = (sectionSpec: any): Section => {
  return {
    id: randomUUID(),
    name: sectionSpec.name ?? 'Section',
    start: sectionSpec.start ?? 0,
    loop: sectionSpec.loop ?? false,
  };
};

const projectFile = process.argv[2];
const spec = JSON.parse(fs.readFileSync(projectFile, 'utf-8'));

const project = emptyProject();
project.info = {
  id: randomUUID(),
  name: spec.name ?? 'Project',
  version: '1',
  lastSaved: Date.now(),
};

project.songs = spec.songs ? spec.songs.map(convertSong) : [];

project.selections = {
  song: project.songs[0].id,
  section: project.songs[0].sections[0].id,
};

console.log(JSON.stringify(project, null, 2));
