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
  const newProject = {...state.project};
  console.log('state = ', state);
  const previousSamplesInUse = getSamplesInProject(newProject);

  switch (action.type) {
    case ADD_SAMPLE: {
      const {sample, songId} = action as AddSampleAction;

      addSampleToSong(
        backend,
        newProject,
        state.projectInfo?.id ?? '',
        songId,
        sample
      );

      break;
    }

    case ADD_SECTION: {
      const {section, songId} = action as AddSectionAction;
      addSection(newProject, songId, section);
      break;
    }

    case ADD_SONG: {
      addSong(newProject);
      break;
    }

    case CREATE_PROJECT: {
      await backend.createProject();
      await backend.fetchProjects();
      break;
    }

    case LOAD_PROJECT: {
      const {projectId} = action as LoadProjectAction;
      await backend.loadProject(projectId);
      break;
    }

    case MOVE_SECTION: {
      const {songId, fromIndex, toIndex} = action as MoveSectionAction;
      moveSection(newProject, songId, fromIndex, toIndex);
      break;
    }

    case MOVE_SONG: {
      const {fromIndex, toIndex} = action as MoveSongAction;
      moveSong(newProject, fromIndex, toIndex);
      break;
    }

    case REMOVE_SAMPLE: {
      const {songId} = action as RemoveSampleAction;
      const song = newProject.songs.find((s) => s.id.equals(songId));
      if (!song || !song.sample) {
        console.error(`Song with ID ${songId} not found or has no sample`);
        break;
      }

      await backend.removeSample(state.projectInfo?.id ?? '', song.sample.id);
      break;
    }

    case REMOVE_PROJECT: {
      const {projectId} = action as RemoveProjectAction;
      await backend.removeProject(projectId);
      await backend.fetchProjects();
      break;
    }

    case REMOVE_SECTION: {
      const {songId, sectionId} = action as RemoveSectionAction;
      removeSection(newProject, songId, sectionId);
      break;
    }

    case REMOVE_SONG: {
      const {songId} = action as RemoveSongAction;
      removeSong(newProject, songId);
      break;
    }

    case RENAME_PROJECT: {
      const {newName} = action as RenameProjectAction;
      await backend.renameProject(state.projectInfo?.id ?? '', newName);
      await backend.fetchProjects();
      break;
    }

    case SELECT_SONG: {
      const {songId} = action as SelectSongAction;
      selectSong(newProject, songId);
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
      updateSection(newProject, songId, newSection);
      break;
    }

    case UPDATE_SONG: {
      const {newSong} = action as UpdateSongAction;
      updateSong(newProject, newSong);
      break;
    }

    default:
      console.error('Unknown action type:', action.type);
      return state;
  }

  const currentSamplesInUse = getSamplesInProject(newProject);

  await garbageCollectSamples(
    backend,
    state.projectInfo?.id ?? '',
    previousSamplesInUse,
    currentSamplesInUse
  );

  await backend.updateProject(state.projectInfo?.id ?? '', newProject);

  console.debug('Updated project:', newProject);

  return {
    ...state,
    project: newProject,
  };
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
