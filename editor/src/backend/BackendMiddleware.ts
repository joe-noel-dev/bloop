import {Project, Sample} from '../api/bloop';
import {ID, randomId} from '../api/helpers';
import {updateSong} from '../api/project-helpers';
import {DispatchFunction, MiddlewareAPI} from '../dispatcher';
import {
  Action,
  ADD_SAMPLE,
  AddSampleAction,
  CREATE_PROJECT,
  LOAD_PROJECT,
  LOAD_PROJECTS,
  LoadProjectAction,
  loadProjectsAction,
  REMOVE_PROJECT,
  RemoveProjectAction,
  RENAME_PROJECT,
  RenameProjectAction,
  resetAppStateAction,
  SAVE_PROJECT,
  setProjectAction,
  setProjectInfoAction,
  setProjectsAction,
  setSaveStateAction,
  SIGN_IN,
  SignInAction,
  SIGN_OUT,
  SignOutAction,
} from '../dispatcher/action';
import {Backend} from './Backend';
import Long from 'long';

export const backendMiddleware =
  (api: MiddlewareAPI) =>
  (next: DispatchFunction) =>
  async (action: Action) => {
    const backend = api.getBackend();
    const state = api.getState();

    switch (action.type) {
      case SIGN_IN: {
        const {userId, password} = action as SignInAction;
        const user = await backend.signIn(userId, password);
        console.debug('Signed in user:', user);
        break;
      }

      case SIGN_OUT: {
        await backend.signOut();
        console.debug('Signed out user');
        // Reset the app state to clear all user data
        api.dispatch(resetAppStateAction());
        break;
      }

      case ADD_SAMPLE: {
        const {sample, songId} = action as AddSampleAction;

        await addSampleToSong(
          backend,
          state.project,
          state.projectInfo?.id ?? '',
          songId,
          sample
        );

        break;
      }

      case CREATE_PROJECT: {
        const [project, info] = await backend.createProject();
        api.dispatch(setProjectInfoAction(info));
        api.dispatch(setProjectAction(project));
        api.dispatch(loadProjectsAction());
        break;
      }

      case LOAD_PROJECT: {
        const {projectId} = action as LoadProjectAction;
        const [project, info] = await backend.loadProject(projectId);
        api.dispatch(setProjectInfoAction(info));
        api.dispatch(setProjectAction(project));
        break;
      }

      case SAVE_PROJECT:
        {
          api.dispatch(setSaveStateAction('saving'));
          try {
            await backend.updateProject(
              state.projectInfo?.id ?? '',
              state.project
            );
            api.dispatch(setSaveStateAction('saved'));

            // Auto-revert to idle after 2 seconds
            setTimeout(() => {
              api.dispatch(setSaveStateAction('idle'));
            }, 2000);
          } catch (error) {
            console.error(`Failed to update project on backend: ${error}`);
            api.dispatch(setSaveStateAction('idle'));
          }
        }
        break;

      case REMOVE_PROJECT: {
        const {projectId} = action as RemoveProjectAction;
        await backend.removeProject(projectId);
        api.dispatch(loadProjectsAction());
        break;
      }

      case RENAME_PROJECT: {
        const {newName} = action as RenameProjectAction;
        const projectInfo = await backend.renameProject(
          state.projectInfo?.id ?? '',
          newName
        );
        api.dispatch(setProjectInfoAction(projectInfo));
        api.dispatch(loadProjectsAction());
        break;
      }

      case LOAD_PROJECTS: {
        const projects = await backend.fetchProjects();
        api.dispatch(setProjectsAction(projects));
        break;
      }
    }

    const previousSamplesInUse = getSamplesInProject(api.getState().project);

    await next(action);

    const currentSamplesInUse = getSamplesInProject(api.getState().project);

    await garbageCollectSamples(
      backend,
      state.projectInfo?.id ?? '',
      previousSamplesInUse,
      currentSamplesInUse
    );
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
