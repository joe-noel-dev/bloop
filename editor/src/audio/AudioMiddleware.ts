import {Middleware} from '../dispatcher/middleware';

export const audioMiddleware: Middleware =
  (api) => (next) => async (action) => {
    await next(action);

    const projectInfo = api.getState().projectInfo;
    const audioController = api.getAudioController();
    if (audioController && projectInfo) {
      audioController.setProjectInfo(projectInfo);
    }

    const project = api.getState().project;
    if (audioController && project) {
      audioController.setProject(project);
    }
  };
