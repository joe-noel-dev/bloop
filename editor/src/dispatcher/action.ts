import {Project, Section, Song} from '../api/bloop';
import {ID} from '../api/helpers';
import {DbProject} from '../backend/Backend';
import {SaveState} from '../state/AppState';
import {SampleState} from '../audio/SampleManager';
import {ThemeMode} from '../state/ThemeState';
import {PlaybackState} from '../audio/AudioController';

// 1:
export const ADD_SAMPLE = 'ADD_SAMPLE';
export const ADD_SECTION = 'ADD_SECTION';
export const ADD_SONG = 'ADD_SONG';
export const CREATE_PROJECT = 'CREATE_PROJECT';
export const LOAD_PROJECT = 'LOAD_PROJECT';
export const SAVE_PROJECT = 'SAVE_PROJECT';
export const MOVE_SECTION = 'MOVE_SECTION';
export const MOVE_SONG = 'MOVE_SONG';
export const REMOVE_PROJECT = 'REMOVE_PROJECT';
export const REMOVE_SAMPLE = 'REMOVE_SAMPLE';
export const REMOVE_SECTION = 'REMOVE_SECTION';
export const REMOVE_SONG = 'REMOVE_SONG';
export const RENAME_PROJECT = 'RENAME_PROJECT';
export const SELECT_SONG = 'SELECT_SONG';
export const SIGN_IN = 'SIGN_IN';
export const SIGN_OUT = 'SIGN_OUT';
export const SPLIT_SECTION = 'SPLIT_SECTION';
export const UPDATE_SECTION = 'UPDATE_SECTION';
export const UPDATE_SONG = 'UPDATE_SONG';
export const LOAD_PROJECTS = 'LOAD_PROJECTS';
export const REMOVE_ALL_SECTIONS = 'REMOVE_ALL_SECTIONS';
export const RESET_APP_STATE = 'RESET_APP_STATE';
export const SET_PROJECT = 'SET_PROJECT';
export const SET_PROJECTS = 'SET_PROJECTS';
export const SET_PROJECT_INFO = 'SET_PROJECT_INFO';
export const PLAY = 'PLAY';
export const STOP = 'STOP';
export const SET_PLAYBACK_STATE = 'SET_PLAYBACK_STATE';
export const SET_SAVE_STATE = 'SET_SAVE_STATE';
export const SET_SAMPLE_STATE = 'SET_SAMPLE_STATE';
export const SET_THEME_MODE = 'SET_THEME_MODE';

// 2:

export const addSampleAction = (songId: ID, sample: File) => ({
  type: ADD_SAMPLE,
  songId,
  sample,
});

export const addSectionAction = (songId: ID, section?: Partial<Section>) => ({
  type: ADD_SECTION,
  songId,
  section,
});

export const addSongAction = () => ({
  type: ADD_SONG,
});

export const createProjectAction = () => ({
  type: CREATE_PROJECT,
});

export const loadProjectAction = (projectId: string) => ({
  type: LOAD_PROJECT,
  projectId,
});

export const saveProjectAction = () => ({
  type: SAVE_PROJECT,
});

export const moveSectionAction = (
  songId: ID,
  fromIndex: number,
  toIndex: number
) => ({
  type: MOVE_SECTION,
  songId,
  fromIndex,
  toIndex,
});

export const moveSongAction = (fromIndex: number, toIndex: number) => ({
  type: MOVE_SONG,
  fromIndex,
  toIndex,
});

export const removeProjectAction = (projectId: string) => ({
  type: REMOVE_PROJECT,
  projectId,
});

export const removeSampleAction = (songId: ID) => ({
  type: REMOVE_SAMPLE,
  songId,
});

export const removeSectionAction = (songId: ID, sectionId: ID) => ({
  type: REMOVE_SECTION,
  songId,
  sectionId,
});

export const removeSongAction = (songId: ID) => ({
  type: REMOVE_SONG,
  songId,
});

export const renameProjectAction = (newName: string) => ({
  type: RENAME_PROJECT,
  newName,
});

export const selectSongAction = (songId: ID) => ({
  type: SELECT_SONG,
  songId,
});

export const signInAction = (userId: string, password: string) => ({
  type: SIGN_IN,
  userId,
  password,
});

export const signOutAction = () => ({
  type: SIGN_OUT,
});

export const updateSectionAction = (songId: ID, newSection: Section) => {
  return {
    type: UPDATE_SECTION,
    songId,
    newSection,
  };
};

export const splitSectionAction = (songId: ID, sectionId: ID) => ({
  type: SPLIT_SECTION,
  songId,
  sectionId,
});

export const updateSongAction = (newSong: Song) => ({
  type: UPDATE_SONG,
  newSong,
});

export const loadProjectsAction = () => ({
  type: LOAD_PROJECTS,
});

export const removeAllSectionsAction = (songId: ID) => ({
  type: REMOVE_ALL_SECTIONS,
  songId,
});

export const resetAppStateAction = () => ({
  type: RESET_APP_STATE,
});

export const setProjectAction = (project: Project) => ({
  type: SET_PROJECT,
  project,
});

export const setProjectsAction = (projects: Array<DbProject>) => ({
  type: SET_PROJECTS,
  projects,
});

export const setProjectInfoAction = (projectInfo: DbProject) => ({
  type: SET_PROJECT_INFO,
  projectInfo,
});

export const playAction = (songId: ID, sectionId: ID) => ({
  type: PLAY,
  songId,
  sectionId,
});

export const stopAction = () => ({
  type: STOP,
});

export const setPlaybackStateAction = (
  playbackState: PlaybackState | null
) => ({
  type: SET_PLAYBACK_STATE,
  playbackState,
});

export const setSaveStateAction = (saveState: SaveState) => ({
  type: SET_SAVE_STATE,
  saveState,
});

export const setSampleStateAction = (
  sampleId: ID,
  sampleState: SampleState
) => ({
  type: SET_SAMPLE_STATE,
  sampleId,
  sampleState,
});

export const setThemeModeAction = (mode: ThemeMode) => ({
  type: SET_THEME_MODE,
  mode,
});

// 3:
export type AddSampleAction = ReturnType<typeof addSampleAction>;
export type AddSectionAction = ReturnType<typeof addSectionAction>;
export type AddSongAction = ReturnType<typeof addSongAction>;
export type CreateProjectAction = ReturnType<typeof createProjectAction>;
export type LoadProjectAction = ReturnType<typeof loadProjectAction>;
export type SaveProjectAction = ReturnType<typeof saveProjectAction>;
export type MoveSectionAction = ReturnType<typeof moveSectionAction>;
export type MoveSongAction = ReturnType<typeof moveSongAction>;
export type RemoveProjectAction = ReturnType<typeof removeProjectAction>;
export type RemoveSampleAction = ReturnType<typeof removeSampleAction>;
export type RemoveSectionAction = ReturnType<typeof removeSectionAction>;
export type RemoveSongAction = ReturnType<typeof removeSongAction>;
export type RenameProjectAction = ReturnType<typeof renameProjectAction>;
export type SelectSongAction = ReturnType<typeof selectSongAction>;
export type SignInAction = ReturnType<typeof signInAction>;
export type SignOutAction = ReturnType<typeof signOutAction>;
export type SplitSectionAction = ReturnType<typeof splitSectionAction>;
export type UpdateSectionAction = ReturnType<typeof updateSectionAction>;
export type UpdateSongAction = ReturnType<typeof updateSongAction>;
export type LoadProjectsAction = ReturnType<typeof loadProjectsAction>;
export type RemoveAllSectionsAction = ReturnType<
  typeof removeAllSectionsAction
>;
export type ResetAppStateAction = ReturnType<typeof resetAppStateAction>;
export type SetProjectAction = ReturnType<typeof setProjectAction>;
export type SetProjectsAction = ReturnType<typeof setProjectsAction>;
export type SetProjectInfoAction = ReturnType<typeof setProjectInfoAction>;
export type PlayAction = ReturnType<typeof playAction>;
export type StopAction = ReturnType<typeof stopAction>;
export type SetPlaybackStateAction = ReturnType<typeof setPlaybackStateAction>;
export type SetSaveStateAction = ReturnType<typeof setSaveStateAction>;
export type SetSampleStateAction = ReturnType<typeof setSampleStateAction>;
export type SetThemeModeAction = ReturnType<typeof setThemeModeAction>;

// 4:
export type Action =
  | AddSampleAction
  | AddSectionAction
  | AddSongAction
  | CreateProjectAction
  | LoadProjectAction
  | SaveProjectAction
  | MoveSectionAction
  | MoveSongAction
  | RemoveProjectAction
  | RemoveSampleAction
  | RemoveSectionAction
  | RemoveSongAction
  | RenameProjectAction
  | SelectSongAction
  | SignInAction
  | SignOutAction
  | SplitSectionAction
  | UpdateSectionAction
  | UpdateSongAction
  | LoadProjectsAction
  | RemoveAllSectionsAction
  | ResetAppStateAction
  | SetProjectAction
  | SetProjectsAction
  | SetProjectInfoAction
  | PlayAction
  | StopAction
  | SetPlaybackStateAction
  | SetSaveStateAction
  | SetSampleStateAction
  | SetThemeModeAction;
