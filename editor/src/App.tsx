import {useEffect, useState} from 'react';
import {AppState, AppStateContext} from './state/AppState';
import CssBaseline from '@mui/joy/CssBaseline';
import {CssVarsProvider} from '@mui/joy/styles';
import {Box} from '@mui/joy';
import {Project as ModelProject} from './api/bloop';
import '@fontsource/inter';
import {Project} from './app/project/Project';
import {LoginScreen} from './app/login/LoginScreen';
import {
  Backend,
  BackendContext,
  createBackend,
  DbProject,
  DbUser,
} from './backend/Backend';
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
    newBackend.fetchProjects();

    newBackend.events.on('user', (user: DbUser | null) => {
      if (user) {
        newBackend.fetchProjects();
      }
    });

    newBackend.events.on('project', (project) => {
      setState({
        ...state,
        project: project as ModelProject,
      });
    });

    newBackend.events.on('projects', (projects: DbProject[]) => {
      setState({
        ...state,
        projects,
      });
    });

    newBackend.events.on('project-info', (projectInfo: DbProject | null) => {
      console.log('Project info set to:', projectInfo);
      setState({
        ...state,
        projectInfo: projectInfo ?? undefined,
      });
    });

    setBackend(newBackend);
  }, []);

  if (!backend) {
    return;
  }

  console.log('App state:', state);
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
