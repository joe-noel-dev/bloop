import {Action} from './action';
import {AppState} from '../state/AppState';
import {Backend} from '../backend/Backend';
import {AudioController} from '../audio/AudioController';

export type DispatchFunction = (action: Action) => Promise<void> | void;

export interface MiddlewareAPI {
  getState: () => AppState;
  getBackend: () => Backend;
  getAudioController: () => AudioController;
  dispatch: DispatchFunction;
}

export type Middleware = (
  api: MiddlewareAPI
) => (next: DispatchFunction) => DispatchFunction;

/**
 * Composes multiple middleware functions into a single enhanced dispatch function
 */
export const applyMiddleware =
  (...middlewares: Middleware[]) =>
  (api: MiddlewareAPI) =>
  (dispatch: DispatchFunction): DispatchFunction => {
    const composedDispatch = middlewares.reduceRight(
      (acc, middleware) => middleware(api)(acc),
      dispatch
    );

    // Return an async-aware dispatch function
    return async (action: Action) => {
      await composedDispatch(action);
    };
  };
