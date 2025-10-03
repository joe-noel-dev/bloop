import {describe, it, expect} from 'vitest';
import {reducer} from './reducer';
import {setSampleStateAction, setThemeModeAction} from './action';
import {createTestAppState} from '../test-utils/app-state-helpers';
import Long from 'long';

describe('Reducer - Sample State', () => {
  const initialState = createTestAppState();

  const testSampleId = Long.fromNumber(12345);

  it('should set sample loading state', () => {
    const action = setSampleStateAction(testSampleId, 'loading');
    const newState = reducer(action, initialState);

    expect(newState.sampleStates.get(testSampleId)).toEqual({
      state: 'loading',
      buffer: undefined,
    });
    expect(newState.sampleStates.size).toBe(1);
  });

  it('should update sample state from loading to converting', () => {
    const loadingAction = setSampleStateAction(testSampleId, 'loading');
    const loadingState = reducer(loadingAction, initialState);

    const convertingAction = setSampleStateAction(testSampleId, 'converting');
    const convertingState = reducer(convertingAction, loadingState);

    expect(convertingState.sampleStates.get(testSampleId)).toEqual({
      state: 'converting',
      buffer: undefined,
    });
    expect(convertingState.sampleStates.size).toBe(1);
  });

  it('should update sample state from converting to loaded', () => {
    const convertingAction = setSampleStateAction(testSampleId, 'converting');
    const convertingState = reducer(convertingAction, initialState);

    // Since we only pass the state now, the buffer would be managed separately
    const loadedAction = setSampleStateAction(testSampleId, 'loaded');
    const loadedState = reducer(loadedAction, convertingState);

    expect(loadedState.sampleStates.get(testSampleId)).toEqual({
      state: 'loaded',
      buffer: undefined, // Buffer is not managed through this action anymore
    });
  });

  it('should set sample error state', () => {
    const action = setSampleStateAction(testSampleId, 'error');
    const newState = reducer(action, initialState);

    expect(newState.sampleStates.get(testSampleId)).toEqual({
      state: 'error',
      buffer: undefined,
    });
  });

  it('should handle multiple samples independently', () => {
    const sampleId1 = Long.fromNumber(123);
    const sampleId2 = Long.fromNumber(456);

    const action1 = setSampleStateAction(sampleId1, 'loading');
    const state1 = reducer(action1, initialState);

    const action2 = setSampleStateAction(sampleId2, 'converting');
    const state2 = reducer(action2, state1);

    expect(state2.sampleStates.get(sampleId1)).toEqual({
      state: 'loading',
      buffer: undefined,
    });
    expect(state2.sampleStates.get(sampleId2)).toEqual({
      state: 'converting',
      buffer: undefined,
    });
    expect(state2.sampleStates.size).toBe(2);
  });

  it('should not mutate the original state', () => {
    const action = setSampleStateAction(testSampleId, 'loading');
    const newState = reducer(action, initialState);

    expect(newState).not.toBe(initialState);
    expect(newState.sampleStates).not.toBe(initialState.sampleStates);
    expect(initialState.sampleStates.size).toBe(0);
    expect(newState.sampleStates.size).toBe(1);
  });
});

describe('Reducer - Theme State', () => {
  const initialState = createTestAppState();

  it('should set theme mode to light', () => {
    const action = setThemeModeAction('light');
    const newState = reducer(action, initialState);

    expect(newState.theme.mode).toBe('light');
    expect(newState.theme.effectiveMode).toBe('light');
  });

  it('should set theme mode to dark', () => {
    const action = setThemeModeAction('dark');
    const newState = reducer(action, initialState);

    expect(newState.theme.mode).toBe('dark');
    expect(newState.theme.effectiveMode).toBe('dark');
  });

  it('should set theme mode to system', () => {
    const action = setThemeModeAction('system');
    const newState = reducer(action, initialState);

    expect(newState.theme.mode).toBe('system');
    // effectiveMode should be either 'light' or 'dark' based on system preference
    expect(['light', 'dark']).toContain(newState.theme.effectiveMode);
  });

  it('should not mutate original state when updating theme', () => {
    const action = setThemeModeAction('dark');
    const newState = reducer(action, initialState);

    expect(newState).not.toBe(initialState);
    expect(newState.theme).not.toBe(initialState.theme);
  });
});
