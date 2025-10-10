import {describe, it, expect} from 'vitest';
import {reducer} from '../dispatcher/reducer';
import {
  showErrorNotificationAction,
  hideErrorNotificationAction,
} from '../dispatcher/action';
import {AppState} from '../state/AppState';
import {createTestAppState} from '../test-utils/app-state-helpers';

describe('Reducer - Error Notifications', () => {
  const initialState = createTestAppState();

  it('should show error notification when action is dispatched', () => {
    const errorMessage = 'Test error message';
    const action = showErrorNotificationAction(errorMessage);
    const newState = reducer(action, initialState);

    expect(newState.errorNotification).toBeDefined();
    expect(newState.errorNotification?.message).toBe(errorMessage);
    expect(newState.errorNotification?.id).toBeDefined();
    expect(newState.errorNotification?.timestamp).toBeDefined();
  });

  it('should hide error notification when hide action is dispatched', () => {
    const stateWithNotification: AppState = {
      ...initialState,
      errorNotification: {
        id: 'test-id',
        message: 'Test error',
        timestamp: Date.now(),
      },
    };

    const action = hideErrorNotificationAction();
    const newState = reducer(action, stateWithNotification);

    expect(newState.errorNotification).toBeUndefined();
  });

  it('should replace existing error notification with new one', () => {
    const firstMessage = 'First error';
    const secondMessage = 'Second error';

    const firstAction = showErrorNotificationAction(firstMessage);
    const stateWithFirstError = reducer(firstAction, initialState);

    const secondAction = showErrorNotificationAction(secondMessage);
    const finalState = reducer(secondAction, stateWithFirstError);

    expect(finalState.errorNotification?.message).toBe(secondMessage);
    expect(finalState.errorNotification?.id).not.toBe(
      stateWithFirstError.errorNotification?.id
    );
  });

  it('should generate unique IDs for different error notifications', () => {
    const message = 'Test error';

    const firstAction = showErrorNotificationAction(message);
    const firstState = reducer(firstAction, initialState);

    const secondAction = showErrorNotificationAction(message);
    const secondState = reducer(secondAction, initialState);

    expect(firstState.errorNotification?.id).not.toBe(
      secondState.errorNotification?.id
    );
  });
});
