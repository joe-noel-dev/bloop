import {SET_PROJECT} from '../dispatcher/action';
import {Middleware} from '../dispatcher/middleware';

export const audioMiddleware: Middleware =
  (api) => (next) => async (action) => {
    await next(action);

    const project = api.getState().project;
    const audioController = api.getAudioController();
    if (audioController && project) {
      audioController.setProject(project);
    }
  };
