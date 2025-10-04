import {AppState, emptyAppState} from '../state/AppState';
import {Project} from '../api/bloop';
import {SampleInCache} from '../audio/SampleManager';
import {ThemeState} from '../state/ThemeState';
import {PlaybackState} from '../audio/AudioController';

/**
 * Creates an empty AppState for testing purposes
 */
export const createTestAppState = (
  overrides: Partial<AppState> = {}
): AppState => ({
  ...emptyAppState(),
  ...overrides,
});

/**
 * Creates a simplified theme state for testing
 */
export const createTestTheme = (
  mode: 'light' | 'dark' = 'light'
): ThemeState => ({
  mode,
  effectiveMode: mode,
});

/**
 * Creates an AppState with a custom project
 */
export const createTestAppStateWithProject = (
  project: Project,
  overrides: Partial<AppState> = {}
): AppState =>
  createTestAppState({
    project,
    ...overrides,
  });

/**
 * Creates an AppState with sample states
 */
export const createTestAppStateWithSamples = (
  sampleStates: Map<Long, SampleInCache>,
  overrides: Partial<AppState> = {}
): AppState =>
  createTestAppState({
    sampleStates,
    ...overrides,
  });

/**
 * Creates an AppState with playing state
 */
export const createTestAppStateWithPlayback = (
  playbackState: PlaybackState | null,
  overrides: Partial<AppState> = {}
): AppState =>
  createTestAppState({
    playbackState,
    ...overrides,
  });
