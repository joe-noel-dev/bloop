import {
  Action,
  ADD_SECTION,
  ADD_SONG,
  AddSectionAction,
  MOVE_SECTION,
  MOVE_SONG,
  MoveSectionAction,
  MoveSongAction,
  REMOVE_ALL_SECTIONS,
  REMOVE_SAMPLE,
  REMOVE_SECTION,
  REMOVE_SONG,
  RemoveAllSectionsAction,
  RemoveSampleAction,
  RemoveSectionAction,
  RemoveSongAction,
  RESET_APP_STATE,
  SELECT_SONG,
  SelectSongAction,
  SET_PLAYBACK_STATE,
  SET_PROJECT,
  SET_PROJECT_INFO,
  SET_PROJECTS,
  SET_SAVE_STATE,
  SET_SAMPLE_STATE,
  SET_THEME_MODE,
  SetPlaybackStateAction,
  SetProjectAction,
  SetProjectInfoAction,
  SetProjectsAction,
  SetSaveStateAction,
  SetSampleStateAction,
  SetThemeModeAction,
  SPLIT_SECTION,
  SplitSectionAction,
  UPDATE_SECTION,
  UPDATE_SONG,
  UpdateSectionAction,
  UpdateSongAction,
} from './action';
import {AppState, emptyAppState} from '../state/AppState';
import {
  addSection,
  addSong,
  moveSection,
  moveSong,
  removeSection,
  removeSong,
  selectSong,
  splitSection,
  updateSection,
  updateSong,
} from '../api/project-helpers';
import Long from 'long';

export const reducer = (action: Action, state: AppState): AppState => {
  const newState = {...state};

  switch (action.type) {
    case ADD_SECTION: {
      const {section, songId} = action as AddSectionAction;
      addSection(newState.project, songId, section);
      break;
    }

    case ADD_SONG: {
      addSong(newState.project);
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

    case SELECT_SONG: {
      const {songId} = action as SelectSongAction;
      selectSong(newState.project, songId);
      break;
    }

    case SPLIT_SECTION: {
      const {songId, sectionId} = action as SplitSectionAction;
      splitSection(newState.project, songId, sectionId);
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

    case REMOVE_ALL_SECTIONS: {
      const {songId} = action as RemoveAllSectionsAction;
      const song = newState.project.songs.find((s) => s.id.equals(songId));
      if (!song) {
        console.error(`Song with ID ${songId} not found`);
        break;
      }

      song.sections = [];
      if (newState.project.selections?.section.equals(songId)) {
        newState.project.selections.section = Long.ZERO;
      }
      break;
    }

    case RESET_APP_STATE: {
      // Reset to initial state using shared function
      const resetState = emptyAppState();
      Object.assign(newState, resetState);
      break;
    }

    case SET_PROJECT: {
      const {project} = action as SetProjectAction;
      newState.project = project;
      console.debug('Project set:', project);
      break;
    }

    case SET_PROJECTS: {
      const {projects} = action as SetProjectsAction;
      newState.projects = projects;
      break;
    }

    case SET_PROJECT_INFO: {
      const {projectInfo} = action as SetProjectInfoAction;
      newState.projectInfo = projectInfo;
      break;
    }

    case SET_PLAYBACK_STATE: {
      const {playing, songId, sectionId} = action as SetPlaybackStateAction;
      newState.playing = playing;
      newState.playingSongId = songId;
      newState.playingSectionId = sectionId;
      break;
    }

    case SET_SAVE_STATE: {
      const {saveState} = action as SetSaveStateAction;
      newState.saveState = saveState;
      break;
    }

    case SET_SAMPLE_STATE: {
      const {sampleId, sampleState} = action as SetSampleStateAction;
      newState.sampleStates = new Map(newState.sampleStates);
      // Create a SampleInCache object from just the state
      const currentSample = newState.sampleStates.get(sampleId);
      const updatedSample = {
        state: sampleState,
        buffer: currentSample?.buffer, // Preserve existing buffer if any
      };
      newState.sampleStates.set(sampleId, updatedSample);
      break;
    }

    case SET_THEME_MODE: {
      const {mode} = action as SetThemeModeAction;

      // Save to localStorage
      try {
        localStorage.setItem('theme-mode', mode);
      } catch (e) {
        // Fallback for environments without localStorage
      }

      // Determine effective mode with safe system preference check
      let systemPrefersDark = false;
      try {
        systemPrefersDark =
          window.matchMedia?.('(prefers-color-scheme: dark)').matches || false;
      } catch (e) {
        // Fallback for environments without matchMedia
        systemPrefersDark = false;
      }

      const effectiveMode =
        mode === 'system' ? (systemPrefersDark ? 'dark' : 'light') : mode;

      newState.theme = {
        mode,
        effectiveMode,
      };
      break;
    }
  }

  return newState;
};
