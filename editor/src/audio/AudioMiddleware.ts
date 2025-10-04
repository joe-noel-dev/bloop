import {PlayAction} from '../dispatcher/action';
import {Middleware} from '../dispatcher/middleware';

export const audioMiddleware: Middleware = (api) => {
  const audioController = api.getAudioController();

  audioController.setDispatch(api.dispatch);

  return (next) => async (action) => {
    if (action.type === 'PLAY') {
      const playAction = action as PlayAction;
      audioController.play(playAction.songId, playAction.sectionId);
    }

    if (action.type === 'STOP') {
      audioController.stop();
    }

    await next(action);

    const projectInfo = api.getState().projectInfo;
    if (projectInfo) {
      audioController.setProjectInfo(projectInfo);
    }

    const project = api.getState().project;
    if (project) {
      audioController.setProject(project);
    }
  };
};
