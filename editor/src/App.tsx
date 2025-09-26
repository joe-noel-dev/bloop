import {useEffect, useState, useRef} from 'react';
import {AppState, AppStateContext} from './state/AppState';
import CssBaseline from '@mui/joy/CssBaseline';
import {CssVarsProvider} from '@mui/joy/styles';
import {Box} from '@mui/joy';
import '@fontsource/inter';
import {Project} from './app/project/Project';
import {LoginScreen} from './app/login/LoginScreen';
import {Backend, BackendContext, createBackend} from './backend/Backend';
import {DispatcherContext} from './dispatcher/dispatcher';
import {Action} from './dispatcher/action';
import {reducer} from './dispatcher/reducer';
import {emptyProject} from './api/project-helpers';
import {applyMiddleware, DispatchFunction} from './dispatcher/middleware';
import {loggingMiddleware} from './dispatcher/loggingMiddleware';
import {AudioControllerContext} from './audio/AudioControllerContext';
import {AudioController, createAudioController} from './audio/AudioController';
import {audioMiddleware} from './audio/AudioMiddleware';
import {backendMiddleware} from './backend/BackendMiddleware';

const App = () => {
  const [backend] = useState<Backend>(createBackend());
  const [state, setState] = useState<AppState>({
    project: emptyProject(),
    projects: [],
    playing: false,
    saveState: 'idle',
    sampleStates: new Map(),
  });
  const stateRef = useRef<AppState>(state);
  const [audioController] = useState<AudioController>(
    createAudioController(backend)
  );

  // Keep ref in sync with state
  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  const user = backend?.getUser();

  useEffect(() => {
    backend.fetchProjects().then((projects) => setState({...state, projects}));
  }, []);

  const baseDispatch = (action: Action) => {
    const newState = reducer(action, stateRef.current);
    stateRef.current = newState;
    setState(newState);
  };

  let middlewareDispatch: DispatchFunction;

  // Create middleware API
  const middlewareAPI = {
    getState: () => stateRef.current,
    getBackend: () => backend,
    getAudioController: () => audioController,
    dispatch: (action: Action) => middlewareDispatch(action),
  };

  const dispatch = applyMiddleware(
    loggingMiddleware,
    audioMiddleware,
    backendMiddleware
  )(middlewareAPI)(baseDispatch);

  middlewareDispatch = dispatch;

  return (
    <CssVarsProvider>
      <CssBaseline />
      <AudioControllerContext.Provider value={audioController}>
        <DispatcherContext.Provider value={dispatch}>
          <BackendContext.Provider value={backend}>
            <AppStateContext.Provider value={state}>
              <Box
                sx={{
                  display: 'flex',
                  flexDirection: 'column',
                  minHeight: '100vh',
                }}
              >
                {!user && <LoginScreen />}
                {user && <Box sx={{flexGrow: 1}}>{<Project />}</Box>}
              </Box>
            </AppStateContext.Provider>
          </BackendContext.Provider>
        </DispatcherContext.Provider>
      </AudioControllerContext.Provider>
    </CssVarsProvider>
  );
};

export default App;
