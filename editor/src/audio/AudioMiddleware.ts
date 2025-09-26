import {PlayAction, setPlaybackStateAction} from '../dispatcher/action';
import {Middleware} from '../dispatcher/middleware';

export const audioMiddleware: Middleware = (api) => {
  // Set up the playback state change callback
  const audioController = api.getAudioController();
  audioController.setPlaybackStateChangeCallback(
    (playing, songId, sectionId) => {
      api.dispatch(setPlaybackStateAction(playing, songId, sectionId));
    }
  );

  return (next) => async (action) => {
    if (action.type === 'PLAY') {
      const playAction = action as PlayAction;
      audioController.play(
        playAction.songId,
        playAction.sectionId,
        playAction.loop
      );
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
