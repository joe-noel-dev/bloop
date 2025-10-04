import {useEffect, useState, useRef, useMemo} from 'react';
import {AppState, AppStateContext, emptyAppState} from './state/AppState';
import {Box} from '@mui/joy';
import '@fontsource/inter';
import {Project} from './app/project/Project';
import {LoginScreen} from './app/login/LoginScreen';
import {Header} from './components/Header';
import {TransportBar} from './components/TransportBar';
import {ThemeWrapper} from './components/ThemeWrapper';
import {Backend, BackendContext, createBackend} from './backend/Backend';
import {DispatcherContext} from './dispatcher/dispatcher';
import {
  Action,
  setThemeModeAction,
  loadProjectsAction,
} from './dispatcher/action';
import {reducer} from './dispatcher/reducer';
import {applyMiddleware, DispatchFunction} from './dispatcher/middleware';
import {loggingMiddleware} from './dispatcher/loggingMiddleware';
import {AudioControllerContext} from './audio/AudioControllerContext';
import {AudioController, createAudioController} from './audio/AudioController';
import {audioMiddleware} from './audio/AudioMiddleware';
import {backendMiddleware} from './backend/BackendMiddleware';

const App = () => {
  const backend = useMemo<Backend>(() => createBackend(), []);
  const [state, setState] = useState<AppState>(emptyAppState());
  const stateRef = useRef<AppState>(state);
  const audioController = useMemo<AudioController>(
    () => createAudioController(backend),
    [backend]
  );

  // Keep ref in sync with state
  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  const user = backend?.getUser();

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

  // Listen for system theme changes when in system mode
  useEffect(() => {
    if (state.theme.mode !== 'system' || !window.matchMedia) return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const handleChange = () => {
      // Re-trigger theme mode action to recalculate effective mode
      dispatch(setThemeModeAction('system'));
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, [state.theme.mode, dispatch]);

  useEffect(() => {
    if (user) {
      dispatch(loadProjectsAction());
    }
  }, []);

  return (
    <AudioControllerContext.Provider value={audioController}>
      <DispatcherContext.Provider value={dispatch}>
        <BackendContext.Provider value={backend}>
          <AppStateContext.Provider value={state}>
            <ThemeWrapper>
              <Box
                sx={{
                  display: 'flex',
                  flexDirection: 'column',
                  minHeight: '100vh',
                  backgroundColor: 'background.body',
                }}
              >
                {!user && <LoginScreen />}
                {user && (
                  <>
                    <Header />
                    <Box sx={{flexGrow: 1, paddingBottom: 7}}>
                      <Project />
                    </Box>
                    <TransportBar />
                  </>
                )}
              </Box>
            </ThemeWrapper>
          </AppStateContext.Provider>
        </BackendContext.Provider>
      </DispatcherContext.Provider>
    </AudioControllerContext.Provider>
  );
};

export default App;
