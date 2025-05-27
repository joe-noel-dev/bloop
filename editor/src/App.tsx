import {useEffect, useState} from 'react';
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

const App = () => {
  const [backend, setBackend] = useState<Backend | null>(null);
  const [state, setState] = useState<AppState>({
    project: emptyProject(),
    projects: [],
  });

  const user = backend?.getUser();

  useEffect(() => {
    const newBackend = createBackend();
    newBackend
      .fetchProjects()
      .then((projects) => setState({...state, projects}));

    setBackend(newBackend);
  }, []);

  if (!backend) {
    return;
  }

  const dispatch = async (action: Action) => {
    const newState = await reducer(action, state, backend);
    setState(newState);
  };

  return (
    <CssVarsProvider>
      <CssBaseline />
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
    </CssVarsProvider>
  );
};

export default App;
