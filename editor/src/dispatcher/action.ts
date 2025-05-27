import {Section, Song} from '../api/bloop';
import {ID} from '../api/helpers';

// 1:
export const ADD_SECTION = 'ADD_SECTION';
export const ADD_SONG = 'ADD_SONG';
export const MOVE_SECTION = 'MOVE_SECTION';
export const MOVE_SONG = 'MOVE_SONG';
export const REMOVE_SECTION = 'REMOVE_SECTION';
export const REMOVE_SONG = 'REMOVE_SONG';
export const SELECT_SONG = 'SELECT_SONG';
export const UPDATE_SECTION = 'UPDATE_SECTION';
export const UPDATE_SONG = 'UPDATE_SONG';

// 2:

export const addSectionAction = (songId: ID, section?: Partial<Section>) => ({
  type: ADD_SECTION,
  songId,
  section,
});

export const addSongAction = () => ({
  type: ADD_SONG,
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

export const removeSectionAction = (songId: ID, sectionId: ID) => ({
  type: REMOVE_SECTION,
  songId,
  sectionId,
});

export const removeSongAction = (songId: ID) => ({
  type: REMOVE_SONG,
  songId,
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
export type AddSectionAction = ReturnType<typeof addSectionAction>;
export type AddSongAction = ReturnType<typeof addSongAction>;
export type MoveSectionAction = ReturnType<typeof moveSectionAction>;
export type MoveSongAction = ReturnType<typeof moveSongAction>;
export type RemoveSectionAction = ReturnType<typeof removeSectionAction>;
export type RemoveSongAction = ReturnType<typeof removeSongAction>;
export type SelectSongAction = ReturnType<typeof selectSongAction>;
export type UpdateSectionAction = ReturnType<typeof updateSectionAction>;
export type UpdateSongAction = ReturnType<typeof updateSongAction>;

// 4:
export type Action =
  | AddSectionAction
  | AddSongAction
  | MoveSectionAction
  | MoveSongAction
  | RemoveSectionAction
  | RemoveSongAction
  | SelectSongAction
  | UpdateSectionAction
  | UpdateSongAction;
