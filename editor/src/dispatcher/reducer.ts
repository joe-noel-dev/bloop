import {
  Action,
  ADD_SAMPLE,
  ADD_SECTION,
  ADD_SONG,
  AddSampleAction,
  AddSectionAction,
  CREATE_PROJECT,
  LOAD_PROJECT,
  LoadProjectAction,
  MOVE_SECTION,
  MOVE_SONG,
  MoveSectionAction,
  MoveSongAction,
  REMOVE_PROJECT,
  REMOVE_SAMPLE,
  REMOVE_SECTION,
  REMOVE_SONG,
  RemoveProjectAction,
  RemoveSampleAction,
  RemoveSectionAction,
  RemoveSongAction,
  RENAME_PROJECT,
  RenameProjectAction,
  SELECT_SONG,
  SelectSongAction,
  SIGN_IN,
  SignInAction,
  UPDATE_SECTION,
  UPDATE_SONG,
  UpdateSectionAction,
  UpdateSongAction,
} from './action';
import {AppState} from '../state/AppState';
import {
  addSection,
  addSong,
  moveSection,
  moveSong,
  removeSection,
  removeSong,
  selectSong,
  updateSection,
  updateSong,
} from '../api/project-helpers';
import {Backend} from '../backend/Backend';
import {Project, Sample} from '../api/bloop';
import {ID, randomId} from '../api/helpers';
import Long from 'long';

export const reducer = async (
  action: Action,
  state: AppState,
  backend: Backend
): Promise<AppState> => {
  const newState = {...state};
  const previousSamplesInUse = getSamplesInProject(newState.project);

  switch (action.type) {
    case ADD_SAMPLE: {
      const {sample, songId} = action as AddSampleAction;

      await addSampleToSong(
        backend,
        newState.project,
        newState.projectInfo?.id ?? '',
        songId,
        sample
      );

      break;
    }

    case ADD_SECTION: {
      const {section, songId} = action as AddSectionAction;
      addSection(newState.project, songId, section);
      break;
    }

    case ADD_SONG: {
      addSong(newState.project);
      break;
    }

    case CREATE_PROJECT: {
      const [project, info] = await backend.createProject();
      newState.project = project;
      newState.projectInfo = info;
      const projects = await backend.fetchProjects();
      newState.projects = projects;
      break;
    }

    case LOAD_PROJECT: {
      const {projectId} = action as LoadProjectAction;
      const [project, info] = await backend.loadProject(projectId);
      newState.project = project;
      newState.projectInfo = info;
      break;
    }

    case MOVE_SECTION: {
      const {songId, fromIndex, toIndex} = action as MoveSectionAction;
      moveSection(newState.project, songId, fromIndex, toIndex);
      break;
    }

    case MOVE_SONG: {
      const {fromIndex, toIndex} = action as MoveSongAction;
      moveSong(newState.project, fromIndex, toIndex);
      break;
    }

    case REMOVE_SAMPLE: {
      const {songId} = action as RemoveSampleAction;
      const song = newState.project.songs.find((s) => s.id.equals(songId));
      if (!song || !song.sample) {
        console.error(`Song with ID ${songId} not found or has no sample`);
        break;
      }

      updateSong(newState.project, {
        ...song,
        sample: undefined,
      });

      break;
    }

    case REMOVE_PROJECT: {
      const {projectId} = action as RemoveProjectAction;
      await backend.removeProject(projectId);
      newState.projects = await backend.fetchProjects();
      break;
    }

    case REMOVE_SECTION: {
      const {songId, sectionId} = action as RemoveSectionAction;
      removeSection(newState.project, songId, sectionId);
      break;
    }

    case REMOVE_SONG: {
      const {songId} = action as RemoveSongAction;
      removeSong(newState.project, songId);
      break;
    }

    case RENAME_PROJECT: {
      const {newName} = action as RenameProjectAction;
      const projectInfo = await backend.renameProject(
        newState.projectInfo?.id ?? '',
        newName
      );
      newState.projectInfo = projectInfo;
      newState.projects = await backend.fetchProjects();
      break;
    }

    case SELECT_SONG: {
      const {songId} = action as SelectSongAction;
      selectSong(newState.project, songId);
      break;
    }

    case SIGN_IN: {
      const {userId, password} = action as SignInAction;
      const user = await backend.signIn(userId, password);
      console.debug('Signed in user:', user);
      break;
    }

    case UPDATE_SECTION: {
      const {songId, newSection} = action as UpdateSectionAction;
      updateSection(newState.project, songId, newSection);
      break;
    }

    case UPDATE_SONG: {
      const {newSong} = action as UpdateSongAction;
      updateSong(newState.project, newSong);
      break;
    }

    default:
      console.error('Unknown action type:', action.type);
      return state;
  }

  const currentSamplesInUse = getSamplesInProject(newState.project);

  await garbageCollectSamples(
    backend,
    newState.projectInfo?.id ?? '',
    previousSamplesInUse,
    currentSamplesInUse
  );

  await backend.updateProject(newState.projectInfo?.id ?? '', newState.project);

  return newState;
};

const addSampleToSong = async (
  backend: Backend,
  project: Project,
  projectId: string,
  songId: ID,
  sample: File
) => {
  const sampleId = randomId();

  const song = project.songs.find((s) => s.id.equals(songId));

  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  await backend.addSample(projectId, sampleId, sample);

  const sampleDetails = await createSampleFromFile(sampleId, sample);

  updateSong(project, {
    ...song,
    tempo: sampleDetails.tempo ?? song.tempo,
    sample: sampleDetails,
  });
};

const getSamplesInProject = (project: Project): Set<Long> =>
  project.songs.reduce((sampleIds, song) => {
    if (song.sample) {
      sampleIds.add(song.sample.id);
    }

    return sampleIds;
  }, new Set<Long>());

const garbageCollectSamples = async (
  backend: Backend,
  projectId: string,
  previous: Set<Long>,
  current: Set<Long>
) => {
  for (const sampleId of previous) {
    if (!current.has(sampleId)) {
      await backend.removeSample(projectId, sampleId);
    }
  }
};

const readSampleStats = async (audioFile: File) => {
  const audioContext = new AudioContext();
  const arrayBuffer = await audioFile.arrayBuffer();
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

  return {
    sampleRate: audioBuffer.sampleRate,
    channelCount: audioBuffer.numberOfChannels,
    sampleCount: audioBuffer.length,
  };
};

const createSampleFromFile = async (
  sampleId: ID,
  file: File
): Promise<Sample> => {
  const sampleStats = await readSampleStats(file);

  return {
    id: sampleId,
    name: file.name,
    tempo: {
      bpm: getTempoFromFileName(file.name),
    },
    sampleRate: sampleStats.sampleRate,
    sampleCount: Long.fromNumber(sampleStats.sampleCount),
    channelCount: sampleStats.channelCount,
  };
};

const getTempoFromFileName = (fileName: string): number => {
  const defaultBPM = 120;
  const minBPM = 30;
  const maxBPM = 300;

  const match = fileName.match(/(\d+)(?:bpm|BPM)/i);
  if (!match) {
    return defaultBPM;
  }

  const bpm = parseInt(match[1], 10);
  if (isNaN(bpm)) {
    return defaultBPM;
  }

  if (bpm < minBPM || bpm > maxBPM) {
    return defaultBPM;
  }

  return bpm;
};
