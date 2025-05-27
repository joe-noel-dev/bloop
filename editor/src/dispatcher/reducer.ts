import {
  Action,
  ADD_SECTION,
  ADD_SONG,
  AddSectionAction,
  MOVE_SECTION,
  MOVE_SONG,
  MoveSectionAction,
  MoveSongAction,
  REMOVE_SECTION,
  REMOVE_SONG,
  RemoveSectionAction,
  RemoveSongAction,
  SELECT_SONG,
  SelectSongAction,
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

export const reducer = (action: Action, state: AppState): AppState => {
  if (!state.project) {
    return state;
  }

  const newProject = {...state.project};

  switch (action.type) {
    case ADD_SECTION: {
      const {section, songId} = action as AddSectionAction;
      addSection(newProject, songId, section);
      break;
    }

    case ADD_SONG: {
      addSong(newProject);
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

    case SELECT_SONG: {
      const {songId} = action as SelectSongAction;
      selectSong(newProject, songId);
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

  return {
    ...state,
    project: newProject,
  };
};
