import {PlayAction} from '../dispatcher/action';
import {Middleware} from '../dispatcher/middleware';

export const audioMiddleware: Middleware =
  (api) => (next) => async (action) => {
    if (action.type === 'PLAY') {
      const playAction = action as PlayAction;
      const audioController = api.getAudioController();
      audioController.play(
        playAction.songId,
        playAction.sectionId,
        playAction.loop
      );
    }

    if (action.type === 'STOP') {
      const audioController = api.getAudioController();
      audioController.stop();
    }

    await next(action);

    const projectInfo = api.getState().projectInfo;
    const audioController = api.getAudioController();
    if (projectInfo) {
      audioController.setProjectInfo(projectInfo);
    }

    const project = api.getState().project;
    if (project) {
      audioController.setProject(project);
    }
  };
