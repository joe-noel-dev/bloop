import {Project, emptyProject} from '../frontend/src/model/project';
import {randomUUID} from 'crypto';
import {Reader as WavReader} from 'wav';
import {Sample} from '../frontend/src/model/sample';
import {Section} from '../frontend/src/model/section';
import {Song} from '../frontend/src/model/song';

import fs from 'fs';
import os from 'os';
import path from 'path';
import process from 'process';

interface WavInfo {
  sampleRate: number;
  frames: number;
  channels: number;
}

const readWavFile = async (path: string): Promise<WavInfo> => {
  return new Promise((resolve, reject) => {
    const wavReader = new WavReader();

    wavReader.on('error', reject);

    wavReader.on('format', (format) => {
      let frames = 0;

      wavReader.on('data', (data) => {
        frames += data.length / format.blockAlign;
      });

      wavReader.on('end', () => {
        resolve({
          channels: format.channels,
          sampleRate: format.sampleRate,
          frames: Math.round(frames),
        });
      });
    });

    fs.createReadStream(path).pipe(wavReader);
  });
};

const convertSample = async (
  samplePath: string,
  samplesDir: string,
  bpm: number
): Promise<Sample> => {
  const wavInfo = await readWavFile(samplePath);
  const sampleId = randomUUID();
  const destinationPath = path.join(samplesDir, `${sampleId}.wav`);
  fs.copyFileSync(samplePath, destinationPath);
  return {
    id: sampleId,
    name: 'Sample',
    tempo: {bpm},
    sampleCount: wavInfo.frames,
    sampleRate: wavInfo.sampleRate,
    channelCount: wavInfo.channels,
  };
};

const convertSong = async (
  songSpec: any,
  samplesDir: string
): Promise<Song> => {
  const bpm = songSpec.tempo ?? 120;
  const song = {
    id: randomUUID(),
    name: songSpec.name ?? 'Song',
    tempo: {bpm},
    sections: songSpec.sections ? songSpec.sections.map(convertSection) : [],
    sample: songSpec.sample
      ? await convertSample(songSpec.sample, samplesDir, bpm)
      : undefined,
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
    metronome: sectionSpec.metronome ?? false,
  };
};

const convertProject = async (
  projectSpec: any,
  uuid: string,
  samplesDir: string
): Promise<Project> => {
  const project = emptyProject();
  project.info = {
    id: uuid,
    name: projectSpec.name ?? 'Project',
    version: '1',
    lastSaved: Date.now(),
  };

  project.songs = projectSpec.songs
    ? await Promise.all(
        projectSpec.songs.map((songSpec: any) =>
          convertSong(songSpec, samplesDir)
        )
      )
    : [];

  project.selections = {
    song: project.songs[0].id,
    section: project.songs[0].sections[0].id,
  };

  return project;
};

const writeProject = async (projectFile: string) => {
  const spec = JSON.parse(fs.readFileSync(projectFile, 'utf-8'));

  const projectUuid = randomUUID();

  const projectsDir = path.join(os.homedir(), 'bloop', 'projects');
  const projectDir = path.join(projectsDir, projectUuid);
  const projectJson = path.join(projectDir, 'project.json');
  const samplesDir = path.join(projectDir, 'samples');

  fs.mkdirSync(projectDir);
  fs.mkdirSync(samplesDir);

  const project = await convertProject(spec, projectUuid, samplesDir);

  fs.writeFileSync(projectJson, JSON.stringify(project, null, 2));

  console.log('Project written to: ', projectDir);
};

const projectFile = process.argv[2];
if (!projectFile) {
  console.error('Usage: npx ts-node [spec-file.json]');
  process.exit(1);
}

writeProject(projectFile);
