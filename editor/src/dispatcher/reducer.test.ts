import { describe, it, expect } from 'vitest';
import { reducer } from './reducer';
import { setSampleStateAction } from './action';
import { AppState } from '../state/AppState';
import { emptyProject } from '../api/project-helpers';
import Long from 'long';

describe('Reducer - Sample State', () => {
  const initialState: AppState = {
    project: emptyProject(),
    projects: [],
    playing: false,
    saveState: 'idle',
    sampleStates: new Map(),
  };

  const testSampleId = Long.fromNumber(12345);

  it('should set sample loading state', () => {
    const action = setSampleStateAction(testSampleId, { state: 'loading' });
    const newState = reducer(action, initialState);

    expect(newState.sampleStates.get(testSampleId)).toEqual({ state: 'loading' });
    expect(newState.sampleStates.size).toBe(1);
  });

  it('should update sample state from loading to converting', () => {
    const loadingAction = setSampleStateAction(testSampleId, { state: 'loading' });
    const loadingState = reducer(loadingAction, initialState);

    const convertingAction = setSampleStateAction(testSampleId, { state: 'converting' });
    const convertingState = reducer(convertingAction, loadingState);

    expect(convertingState.sampleStates.get(testSampleId)).toEqual({ state: 'converting' });
    expect(convertingState.sampleStates.size).toBe(1);
  });

  it('should update sample state from converting to loaded', () => {
    const convertingAction = setSampleStateAction(testSampleId, { state: 'converting' });
    const convertingState = reducer(convertingAction, initialState);

    const mockAudioBuffer = {} as AudioBuffer; // Mock AudioBuffer
    const loadedAction = setSampleStateAction(testSampleId, { 
      state: 'loaded', 
      buffer: mockAudioBuffer 
    });
    const loadedState = reducer(loadedAction, convertingState);

    expect(loadedState.sampleStates.get(testSampleId)).toEqual({ 
      state: 'loaded', 
      buffer: mockAudioBuffer 
    });
  });

  it('should set sample error state', () => {
    const action = setSampleStateAction(testSampleId, { state: 'error' });
    const newState = reducer(action, initialState);

    expect(newState.sampleStates.get(testSampleId)).toEqual({ state: 'error' });
  });

  it('should handle multiple samples independently', () => {
    const sampleId1 = Long.fromNumber(123);
    const sampleId2 = Long.fromNumber(456);

    const action1 = setSampleStateAction(sampleId1, { state: 'loading' });
    const state1 = reducer(action1, initialState);

    const action2 = setSampleStateAction(sampleId2, { state: 'converting' });
    const state2 = reducer(action2, state1);

    expect(state2.sampleStates.get(sampleId1)).toEqual({ state: 'loading' });
    expect(state2.sampleStates.get(sampleId2)).toEqual({ state: 'converting' });
    expect(state2.sampleStates.size).toBe(2);
  });

  it('should not mutate the original state', () => {
    const action = setSampleStateAction(testSampleId, { state: 'loading' });
    const newState = reducer(action, initialState);

    expect(newState).not.toBe(initialState);
    expect(newState.sampleStates).not.toBe(initialState.sampleStates);
    expect(initialState.sampleStates.size).toBe(0);
    expect(newState.sampleStates.size).toBe(1);
  });
});