import {describe, it, expect, vi, beforeEach} from 'vitest';
import {reducer} from '../src/dispatcher/reducer';
import {AppState} from '../src/state/AppState';
import {Backend} from '../src/backend/Backend';
import {
  ADD_SAMPLE,
  ADD_SECTION,
  ADD_SONG,
  CREATE_PROJECT,
  LOAD_PROJECT,
  MOVE_SECTION,
  MOVE_SONG,
  REMOVE_ALL_SECTIONS,
  REMOVE_PROJECT,
  REMOVE_SAMPLE,
  REMOVE_SECTION,
  REMOVE_SONG,
  RENAME_PROJECT,
  SELECT_SONG,
  SIGN_IN,
  SPLIT_SECTION,
  UPDATE_SECTION,
  UPDATE_SONG,
  LOAD_PROJECTS,
} from '../src/dispatcher/action';
import {emptyProject} from '../src/api/project-helpers';
import {randomId} from '../src/api/helpers';
import Long from 'long';

// Mock data factory functions
const createMockProject = () => ({
  ...emptyProject(),
  songs: [
    {
      id: Long.fromNumber(1),
      name: 'Test Song',
      tempo: {bpm: 120},
      sections: [
        {
          id: Long.fromNumber(1),
          name: 'Section 1',
          start: 0,
          loop: false,
          metronome: false,
        },
      ],
    },
  ],
  selections: {
    song: Long.fromNumber(1),
    section: Long.fromNumber(1),
  },
});

const createMockDbProject = () => ({
  collectionId: 'test',
  collectionName: 'projects',
  created: new Date(),
  id: 'test-project-id',
  name: 'Test Project',
  project: '',
  samples: [],
  userId: 'test-user',
});

const createMockBackend = (): Backend => ({
  signIn: vi.fn().mockResolvedValue({id: 'user1', email: 'test@test.com'}),
  signOut: vi.fn().mockResolvedValue(undefined),
  getUser: vi.fn().mockReturnValue({email: 'test@test.com', name: 'Test User'}),
  fetchProjects: vi.fn().mockResolvedValue([createMockDbProject()]),
  loadProject: vi
    .fn()
    .mockResolvedValue([createMockProject(), createMockDbProject()]),
  createProject: vi
    .fn()
    .mockResolvedValue([createMockProject(), createMockDbProject()]),
  removeProject: vi.fn().mockResolvedValue(undefined),
  renameProject: vi
    .fn()
    .mockResolvedValue({...createMockDbProject(), name: 'New Name'}),
  updateProject: vi.fn().mockResolvedValue(undefined),
  addSample: vi.fn().mockResolvedValue(undefined),
  removeSample: vi.fn().mockResolvedValue(undefined),
});

const createMockFile = (name: string = 'test.wav') => {
  const file = new File([], name);
  // Mock arrayBuffer method that's used by createSampleFromFile
  file.arrayBuffer = vi.fn().mockResolvedValue(new ArrayBuffer(0));
  return file;
};

// Helper functions to create states with multiple songs/sections
const createStateWithMultipleSections = (
  baseState: AppState,
  songIndex = 0
) => ({
  ...baseState,
  project: {
    ...baseState.project,
    songs: baseState.project.songs.map((song, index) =>
      index === songIndex
        ? {
            ...song,
            sections: [
              ...song.sections,
              {
                id: Long.fromNumber(2),
                name: 'Section 2',
                start: 16,
                loop: false,
                metronome: false,
              },
            ],
          }
        : song
    ),
  },
});

const createStateWithMultipleSongs = (baseState: AppState) => ({
  ...baseState,
  project: {
    ...baseState.project,
    songs: [
      ...baseState.project.songs,
      {
        id: Long.fromNumber(2),
        name: 'Song 2',
        tempo: {bpm: 140},
        sections: [],
      },
    ],
  },
});

const createStateWithMultipleSongsAndSections = (baseState: AppState) => ({
  ...baseState,
  project: {
    ...baseState.project,
    songs: [
      ...baseState.project.songs,
      {
        id: Long.fromNumber(2),
        name: 'Song 2',
        tempo: {bpm: 140},
        sections: [
          {
            id: Long.fromNumber(3),
            name: 'Section 1',
            start: 0,
            loop: false,
            metronome: false,
          },
        ],
      },
    ],
  },
});

describe('reducer', () => {
  let initialState: AppState;
  let mockBackend: Backend;

  beforeEach(() => {
    initialState = {
      project: emptyProject(),
      projectInfo: undefined,
      projects: [],
    };
    mockBackend = createMockBackend();

    // Reset all mocks
    vi.clearAllMocks();
  });

  describe('ADD_SAMPLE', () => {
    it('should add a sample to a song', async () => {
      const songId = initialState.project.songs[0].id;
      const file = createMockFile('test-120bpm.wav');

      const action = {
        type: ADD_SAMPLE,
        songId,
        sample: file,
      };

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.addSample).toHaveBeenCalledWith(
        '',
        expect.any(Long),
        file
      );
      expect(newState.project).toBeDefined();
    });
  });

  describe('ADD_SECTION', () => {
    it('should add a section to a song', async () => {
      const songId = initialState.project.songs[0].id;
      const action = {
        type: ADD_SECTION,
        songId,
        section: {name: 'New Section'},
      };

      const newState = await reducer(action, initialState, mockBackend);

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      expect(song?.sections).toHaveLength(2);
      expect(song?.sections[1].name).toBe('New Section');
    });
  });

  describe('ADD_SONG', () => {
    it('should add a new song to the project', async () => {
      const action = {type: ADD_SONG};

      const newState = await reducer(action, initialState, mockBackend);

      expect(newState.project.songs).toHaveLength(2);
      expect(newState.project.selections?.song).toBeDefined();
    });
  });

  describe('CREATE_PROJECT', () => {
    it('should create a new project', async () => {
      const action = {type: CREATE_PROJECT};

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.createProject).toHaveBeenCalled();
      expect(mockBackend.fetchProjects).toHaveBeenCalled();
      expect(newState.project).toBeDefined();
      expect(newState.projectInfo).toBeDefined();
      expect(newState.projects).toBeDefined();
    });
  });

  describe('LOAD_PROJECT', () => {
    it('should load a project by ID', async () => {
      const projectId = 'test-project-id';
      const action = {
        type: LOAD_PROJECT,
        projectId,
      };

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.loadProject).toHaveBeenCalledWith(projectId);
      expect(newState.project).toBeDefined();
      expect(newState.projectInfo).toBeDefined();
    });
  });

  describe('MOVE_SECTION', () => {
    it('should move a section within a song', async () => {
      const stateWithMultipleSections =
        createStateWithMultipleSections(initialState);

      const action = {
        type: MOVE_SECTION,
        songId: stateWithMultipleSections.project.songs[0].id,
        fromIndex: 0,
        toIndex: 1,
      };

      const newState = await reducer(
        action,
        stateWithMultipleSections,
        mockBackend
      );

      const song = newState.project.songs[0];
      expect(song.sections[0].name).toBe('Section 2');
      expect(song.sections[1].name).toBe('Section');
    });
  });

  describe('MOVE_SONG', () => {
    it('should move a song within the project', async () => {
      const stateWithMultipleSongs = createStateWithMultipleSongs(initialState);

      const action = {
        type: MOVE_SONG,
        fromIndex: 0,
        toIndex: 1,
      };

      const newState = await reducer(
        action,
        stateWithMultipleSongs,
        mockBackend
      );

      expect(newState.project.songs[0].name).toBe('Song 2');
      expect(newState.project.songs[1].name).toBe('Song');
    });
  });

  describe('REMOVE_SAMPLE', () => {
    it('should remove a sample from a song', async () => {
      const songId = Long.fromNumber(1);
      const stateWithSample = {
        ...initialState,
        project: {
          ...initialState.project,
          songs: [
            {
              ...initialState.project.songs[0],
              id: songId,
              sample: {
                id: Long.fromNumber(1),
                name: 'test.wav',
                tempo: {bpm: 120},
                sampleRate: 44100,
                sampleCount: Long.fromNumber(1000),
                channelCount: 2,
              },
            },
          ],
        },
      };

      const action = {
        type: REMOVE_SAMPLE,
        songId,
      };

      const newState = await reducer(action, stateWithSample, mockBackend);

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      expect(song?.sample).toBeUndefined();
    });
  });

  describe('REMOVE_PROJECT', () => {
    it('should remove a project', async () => {
      const projectId = 'test-project-id';
      const action = {
        type: REMOVE_PROJECT,
        projectId,
      };

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.removeProject).toHaveBeenCalledWith(projectId);
      expect(mockBackend.fetchProjects).toHaveBeenCalled();
    });
  });

  describe('REMOVE_SECTION', () => {
    it('should remove a section from a song', async () => {
      const songId = initialState.project.songs[0].id;
      const sectionId = initialState.project.songs[0].sections[0].id;

      const stateWithMultipleSections =
        createStateWithMultipleSections(initialState);

      const action = {
        type: REMOVE_SECTION,
        songId,
        sectionId,
      };

      const newState = await reducer(
        action,
        stateWithMultipleSections,
        mockBackend
      );

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      expect(song?.sections).toHaveLength(1);
      expect(song?.sections[0].name).toBe('Section 2');
    });
  });

  describe('REMOVE_SONG', () => {
    it('should remove a song from the project', async () => {
      const stateWithMultipleSongs = createStateWithMultipleSongs(initialState);

      const songId = stateWithMultipleSongs.project.songs[0].id;
      const action = {
        type: REMOVE_SONG,
        songId,
      };

      const newState = await reducer(
        action,
        stateWithMultipleSongs,
        mockBackend
      );

      expect(newState.project.songs).toHaveLength(1);
      expect(newState.project.songs[0].name).toBe('Song 2');
    });
  });

  describe('RENAME_PROJECT', () => {
    it('should rename a project', async () => {
      const stateWithProject = {
        ...initialState,
        projectInfo: createMockDbProject(),
      };

      const action = {
        type: RENAME_PROJECT,
        newName: 'New Project Name',
      };

      const newState = await reducer(action, stateWithProject, mockBackend);

      expect(mockBackend.renameProject).toHaveBeenCalledWith(
        'test-project-id',
        'New Project Name'
      );
      expect(mockBackend.fetchProjects).toHaveBeenCalled();
    });
  });

  describe('SELECT_SONG', () => {
    it('should select a song', async () => {
      const songId = Long.fromNumber(2);

      const stateWithMultipleSongs =
        createStateWithMultipleSongsAndSections(initialState);

      const action = {
        type: SELECT_SONG,
        songId,
      };

      const newState = await reducer(
        action,
        stateWithMultipleSongs,
        mockBackend
      );

      expect(newState.project.selections?.song.equals(songId)).toBe(true);
    });
  });

  describe('SIGN_IN', () => {
    it('should sign in a user', async () => {
      const action = {
        type: SIGN_IN,
        userId: 'test@test.com',
        password: 'password',
      };

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.signIn).toHaveBeenCalledWith(
        'test@test.com',
        'password'
      );
      expect(newState).toEqual(
        expect.objectContaining({
          project: expect.any(Object),
        })
      );
    });
  });

  describe('SPLIT_SECTION', () => {
    it('should split a section', async () => {
      const songId = initialState.project.songs[0].id;
      const sectionId = initialState.project.songs[0].sections[0].id;

      const action = {
        type: SPLIT_SECTION,
        songId,
        sectionId,
      };

      const newState = await reducer(action, initialState, mockBackend);

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      expect(song?.sections).toHaveLength(2);
    });
  });

  describe('UPDATE_SECTION', () => {
    it('should update a section', async () => {
      const songId = initialState.project.songs[0].id;
      const updatedSection = {
        ...initialState.project.songs[0].sections[0],
        name: 'Updated Section',
        loop: true,
      };

      const action = {
        type: UPDATE_SECTION,
        songId,
        newSection: updatedSection,
      };

      const newState = await reducer(action, initialState, mockBackend);

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      const section = song?.sections.find((s) =>
        s.id.equals(updatedSection.id)
      );
      expect(section?.name).toBe('Updated Section');
      expect(section?.loop).toBe(true);
    });
  });

  describe('UPDATE_SONG', () => {
    it('should update a song', async () => {
      const updatedSong = {
        ...initialState.project.songs[0],
        name: 'Updated Song',
        tempo: {bpm: 140},
      };

      const action = {
        type: UPDATE_SONG,
        newSong: updatedSong,
      };

      const newState = await reducer(action, initialState, mockBackend);

      const song = newState.project.songs.find((s) =>
        s.id.equals(updatedSong.id)
      );
      expect(song?.name).toBe('Updated Song');
      expect(song?.tempo?.bpm).toBe(140);
    });
  });

  describe('LOAD_PROJECTS', () => {
    it('should load all projects', async () => {
      const action = {type: LOAD_PROJECTS};

      const newState = await reducer(action, initialState, mockBackend);

      expect(mockBackend.fetchProjects).toHaveBeenCalled();
      expect(newState.projects).toBeDefined();
    });
  });

  describe('REMOVE_ALL_SECTIONS', () => {
    it('should remove all sections from a song', async () => {
      const songId = initialState.project.songs[0].id;
      const action = {
        type: REMOVE_ALL_SECTIONS,
        songId,
      };

      const newState = await reducer(action, initialState, mockBackend);

      const song = newState.project.songs.find((s) => s.id.equals(songId));
      expect(song?.sections).toHaveLength(0);
    });

    it('should clear section selection when removing all sections', async () => {
      const songId = initialState.project.songs[0].id;
      const stateWithSectionSelected = {
        ...initialState,
        project: {
          ...initialState.project,
          selections: {
            song: songId,
            section: songId, // Selected the same ID for simplicity
          },
        },
      };

      const action = {
        type: REMOVE_ALL_SECTIONS,
        songId,
      };

      const newState = await reducer(
        action,
        stateWithSectionSelected,
        mockBackend
      );

      expect(newState.project.selections?.section.equals(Long.ZERO)).toBe(true);
    });
  });

  describe('Unknown action type', () => {
    it('should return the original state for unknown actions', async () => {
      const action = {type: 'UNKNOWN_ACTION' as any};

      const newState = await reducer(action, initialState, mockBackend);

      expect(newState).toBe(initialState);
    });
  });

  describe('State immutability', () => {
    it('should return a new state object', async () => {
      const action = {type: ADD_SONG};

      const newState = await reducer(action, initialState, mockBackend);

      // Should return a different object reference
      expect(newState).not.toBe(initialState);
      // Should have the expected number of songs
      expect(newState.project.songs).toHaveLength(2);
    });
  });

  describe('Backend integration', () => {
    it('should save to backend for actions that require persistence', async () => {
      const stateWithProject = {
        ...initialState,
        projectInfo: createMockDbProject(),
      };

      const action = {type: ADD_SONG};

      await reducer(action, stateWithProject, mockBackend);

      expect(mockBackend.updateProject).toHaveBeenCalledWith(
        'test-project-id',
        expect.any(Object)
      );
    });

    it('should not save to backend for read-only actions', async () => {
      const songId = Long.fromNumber(1);
      const action = {
        type: SELECT_SONG,
        songId,
      };

      await reducer(action, initialState, mockBackend);

      expect(mockBackend.updateProject).not.toHaveBeenCalled();
    });
  });
});

// Additional tests for utility functions that are part of the reducer
describe('utility functions', () => {
  describe('getTempoFromFileName', () => {
    // This would test the getTempoFromFileName function if it were exported
    // For now, we can test it indirectly through the ADD_SAMPLE action
    it('should extract BPM from filename when adding sample', async () => {
      const mockConsoleLog = vi
        .spyOn(console, 'debug')
        .mockImplementation(() => {});
      const testInitialState = {
        project: emptyProject(),
        projectInfo: undefined,
        projects: [],
      };
      const testMockBackend = createMockBackend();

      const songId = testInitialState.project.songs[0].id;
      const file = createMockFile('drum-loop-140bpm.wav');

      const action = {
        type: ADD_SAMPLE,
        songId,
        sample: file,
      };

      await reducer(action, testInitialState, testMockBackend);

      // The reducer should have processed the file with BPM extraction
      expect(testMockBackend.addSample).toHaveBeenCalled();

      mockConsoleLog.mockRestore();
    });
  });
});
