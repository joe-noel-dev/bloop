import {Section, Song} from '../api/bloop';
import {ID} from '../api/helpers';

// 1:
export const ADD_SAMPLE = 'ADD_SAMPLE';
export const ADD_SECTION = 'ADD_SECTION';
export const ADD_SONG = 'ADD_SONG';
export const CREATE_PROJECT = 'CREATE_PROJECT';
export const LOAD_PROJECT = 'LOAD_PROJECT';
export const MOVE_SECTION = 'MOVE_SECTION';
export const MOVE_SONG = 'MOVE_SONG';
export const REMOVE_PROJECT = 'REMOVE_PROJECT';
export const REMOVE_SAMPLE = 'REMOVE_SAMPLE';
export const REMOVE_SECTION = 'REMOVE_SECTION';
export const REMOVE_SONG = 'REMOVE_SONG';
export const RENAME_PROJECT = 'RENAME_PROJECT';
export const SELECT_SONG = 'SELECT_SONG';
export const SIGN_IN = 'SIGN_IN';
export const UPDATE_SECTION = 'UPDATE_SECTION';
export const UPDATE_SONG = 'UPDATE_SONG';

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

export const updateSectionAction = (songId: ID, newSection: Section) => {
  return {
    type: UPDATE_SECTION,
    songId,
    newSection,
  };
};

export const updateSongAction = (newSong: Song) => ({
  type: UPDATE_SONG,
  newSong,
});

// 3:
export type AddSampleAction = ReturnType<typeof addSampleAction>;
export type AddSectionAction = ReturnType<typeof addSectionAction>;
export type AddSongAction = ReturnType<typeof addSongAction>;
export type CreateProjectAction = ReturnType<typeof createProjectAction>;
export type LoadProjectAction = ReturnType<typeof loadProjectAction>;
export type MoveSectionAction = ReturnType<typeof moveSectionAction>;
export type MoveSongAction = ReturnType<typeof moveSongAction>;
export type RemoveProjectAction = ReturnType<typeof removeProjectAction>;
export type RemoveSampleAction = ReturnType<typeof removeSampleAction>;
export type RemoveSectionAction = ReturnType<typeof removeSectionAction>;
export type RemoveSongAction = ReturnType<typeof removeSongAction>;
export type RenameProjectAction = ReturnType<typeof renameProjectAction>;
export type SelectSongAction = ReturnType<typeof selectSongAction>;
export type SignInAction = ReturnType<typeof signInAction>;
export type UpdateSectionAction = ReturnType<typeof updateSectionAction>;
export type UpdateSongAction = ReturnType<typeof updateSongAction>;

// 4:
export type Action =
  | AddSampleAction
  | AddSectionAction
  | AddSongAction
  | CreateProjectAction
  | LoadProjectAction
  | MoveSectionAction
  | MoveSongAction
  | RemoveProjectAction
  | RemoveSampleAction
  | RemoveSectionAction
  | RemoveSongAction
  | RenameProjectAction
  | SelectSongAction
  | SignInAction
  | UpdateSectionAction
  | UpdateSongAction;
