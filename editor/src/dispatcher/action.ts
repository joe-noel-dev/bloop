import {Section, Song} from '../api/bloop';
import {ID} from '../api/helpers';

// 1:
export const ADD_SONG = 'ADD_SONG';
export const ADD_SECTION = 'ADD_SECTION';
export const MOVE_SONG = 'MOVE_SONG';
export const SELECT_SONG = 'SELECT_SONG';
export const UPDATE_SECTION = 'UPDATE_SECTION';
export const UPDATE_SONG = 'UPDATE_SONG';

// 2:
export const addSongAction = () => ({
  type: ADD_SONG,
});

export const addSectionAction = (songId: ID) => ({
  type: ADD_SECTION,
  songId,
});

export const moveSongAction = (fromIndex: number, toIndex: number) => ({
  type: MOVE_SONG,
  fromIndex,
  toIndex,
});

export const selectSongAction = (songId: ID) => ({
  type: SELECT_SONG,
  songId,
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
export type AddSongAction = ReturnType<typeof addSongAction>;
export type AddSectionAction = ReturnType<typeof addSectionAction>;
export type MoveSongAction = ReturnType<typeof moveSongAction>;
export type SelectSongAction = ReturnType<typeof selectSongAction>;
export type UpdateSectionAction = ReturnType<typeof updateSectionAction>;
export type UpdateSongAction = ReturnType<typeof updateSongAction>;

// 4:
export type Action =
  | AddSongAction
  | AddSectionAction
  | MoveSongAction
  | SelectSongAction
  | UpdateSectionAction
  | UpdateSongAction;
