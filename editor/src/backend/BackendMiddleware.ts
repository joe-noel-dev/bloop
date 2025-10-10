import {Project, Sample} from '../api/bloop';
import {ID, randomId} from '../api/helpers';
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
  updateSongAction,
  showErrorNotificationAction,
} from '../dispatcher/action';
import {Backend, DbProject} from './Backend';
import Long from 'long';

// Track pending requests by unique identifiers to prevent duplicates
const pendingRequests = new Set<string>();

const createRequestId = (
  type: string,
  ...params: (string | number)[]
): string => {
  return `${type}:${params.join(':')}`;
};

export const backendMiddleware =
  (api: MiddlewareAPI) =>
  (next: DispatchFunction) =>
  async (action: Action) => {
    const backend = api.getBackend();
    const state = api.getState();

    switch (action.type) {
      case SIGN_IN: {
        const {userId, password} = action as SignInAction;

        try {
          const user = await backend.signIn(userId, password);
          console.debug('Signed in user:', user);
        } catch (error) {
          console.error('Failed to sign in:', error);
          api.dispatch(
            showErrorNotificationAction(
              'Failed to sign in. Please check your credentials and try again.'
            )
          );
        }

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

        if (!state.projectInfo) {
          api.dispatch(
            showErrorNotificationAction(
              'Create or load a project to add a sample.'
            )
          );
          break;
        }

        try {
          const sampleDetails = await addSampleToSong(
            backend,
            state.project,
            state.projectInfo.id,
            songId,
            sample
          );

          if (!sampleDetails) {
            throw new Error('Sample details are undefined');
          }

          const song = state.project.songs.find((s) => s.id.equals(songId));

          if (!song) {
            throw new Error(`Song with ID ${songId} not found`);
          }

          api.dispatch(
            updateSongAction({
              ...song,
              tempo: sampleDetails.tempo ?? song.tempo,
              sample: sampleDetails,
            })
          );
        } catch (error) {
          console.error('Failed to add sample:', error);
          api.dispatch(
            showErrorNotificationAction(
              'Failed to add sample. Please try again.'
            )
          );
        }

        break;
      }

      case CREATE_PROJECT: {
        try {
          const [project, info] = await backend.createProject();
          api.dispatch(setProjectInfoAction(info));
          api.dispatch(setProjectAction(project));
          api.dispatch(loadProjectsAction());
        } catch (error) {
          console.error('Failed to create project:', error);
          api.dispatch(
            showErrorNotificationAction(
              'Failed to create project. Please try again.'
            )
          );
        }
        break;
      }

      case LOAD_PROJECT: {
        const {projectId} = action as LoadProjectAction;

        const requestId = createRequestId('LOAD_PROJECT', projectId);

        try {
          // Prevent duplicate concurrent requests for the same project
          if (pendingRequests.has(requestId)) {
            console.debug(
              `Project load for ${projectId} already in progress, skipping duplicate request`
            );
            break;
          }

          pendingRequests.add(requestId);
          const [project, info] = await backend.loadProject(projectId);
          api.dispatch(setProjectInfoAction(info));
          api.dispatch(setProjectAction(project));
        } catch (error) {
          console.error(`Failed to load project ${projectId}:`, error);
          api.dispatch(
            showErrorNotificationAction(
              `Failed to load project. Please try again.`
            )
          );
        } finally {
          pendingRequests.delete(requestId);
        }
        break;
      }

      case SAVE_PROJECT:
        {
          if (!state.projectInfo) {
            console.error('No project info available. Cannot save project.');
            break;
          }

          const projectId = state.projectInfo.id;
          const requestId = createRequestId('SAVE_PROJECT', projectId);

          // Prevent duplicate concurrent saves for the same project
          if (pendingRequests.has(requestId)) {
            console.debug(
              `Project save for ${projectId} already in progress, skipping duplicate request`
            );
            break;
          }

          api.dispatch(setSaveStateAction('saving'));

          try {
            await removeUnusedSamples(
              state.project,
              state.projectInfo,
              backend
            );

            pendingRequests.add(requestId);
            await backend.updateProject(projectId, state.project);
            api.dispatch(setSaveStateAction('saved'));

            // Auto-revert to idle after 2 seconds
            setTimeout(() => {
              api.dispatch(setSaveStateAction('idle'));
            }, 2000);
          } catch (error) {
            console.error(`Failed to update project on backend: ${error}`);
            api.dispatch(setSaveStateAction('idle'));
            api.dispatch(
              showErrorNotificationAction(
                'Failed to save project. Please try again.'
              )
            );
          } finally {
            pendingRequests.delete(requestId);
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
        if (!state.projectInfo) {
          console.error('No project info available. Cannot rename project.');
          break;
        }

        const {newName} = action as RenameProjectAction;

        const projectInfo = await backend.renameProject(
          state.projectInfo.id,
          newName
        );

        api.dispatch(setProjectInfoAction(projectInfo));
        api.dispatch(loadProjectsAction());
        break;
      }

      case LOAD_PROJECTS: {
        const requestId = createRequestId('LOAD_PROJECTS');

        // Prevent duplicate concurrent requests
        if (pendingRequests.has(requestId)) {
          console.debug(
            'Projects fetch already in progress, skipping duplicate request'
          );
          break;
        }

        try {
          pendingRequests.add(requestId);
          const projects = await backend.fetchProjects();
          api.dispatch(setProjectsAction(projects));
        } catch (error) {
          console.error('Failed to fetch projects:', error);
          api.dispatch(
            showErrorNotificationAction(
              'Failed to fetch projects. Please try again.'
            )
          );
        } finally {
          pendingRequests.delete(requestId);
        }
        break;
      }
    }

    await next(action);
  };

const addSampleToSong = async (
  backend: Backend,
  project: Project,
  projectId: string,
  songId: ID,
  sample: File
): Promise<Sample | undefined> => {
  const sampleId = randomId();

  const song = project.songs.find((s) => s.id.equals(songId));

  if (!song) {
    console.error(`Song with ID ${songId} not found`);
    return;
  }

  await backend.addSample(projectId, sampleId, sample);

  return await createSampleFromFile(sampleId, sample);
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

const getSamplesInProject = (project: Project): Set<ID> => {
  const samples = new Set<ID>();
  for (const song of project.songs) {
    if (song.sample) {
      samples.add(song.sample.id);
    }
  }
  return samples;
};

const removeUnusedSamples = async (
  project: Project,
  projectInfo: DbProject,
  backend: Backend
) => {
  const samplesInUse = getSamplesInProject(project);

  for (const sampleIdString in projectInfo.samples) {
    const sampleId = backend.getIdFromSampleFileName(sampleIdString);
    if (sampleId && !samplesInUse.has(sampleId)) {
      console.debug(`Removing unused sample ${sampleId} from backend`);
      await backend.removeSample(projectInfo.id, sampleId);
    }
  }
};
