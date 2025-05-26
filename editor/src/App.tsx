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
import {DispatcherContext} from './dispatcher/Dispatcher';
import {Action, ADD_SONG} from './dispatcher/action';
import {addSong} from './api/project-helpers';

const App = () => {
  const [backend, setBackend] = useState<Backend | null>(null);
  const [state, setState] = useState<AppState>({
    project: undefined,
    projectInfo: null,
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
      setState((prevState) => ({
        ...prevState,
        project: project as ModelProject,
      }));
    });
    newBackend.events.on('projects', (projects: DbProject[]) => {
      setState((prevState) => ({
        ...prevState,
        projects: projects,
      }));
    });

    newBackend.events.on('project-info', (projectInfo: DbProject | null) => {
      setState((prevState) => ({
        ...prevState,
        projectInfo: projectInfo,
      }));
    });

    setBackend(newBackend);
  }, []);

  if (!backend) {
    return;
  }

  const dispatcher = (action: Action) => {
    if (!state.project) {
      console.error('No project selected');
      return;
    }

    switch (action.type) {
      case ADD_SONG:
        const newProject = addSong(state.project);
        setState({
          ...state,
          project: newProject,
        });
        break;
      default:
        console.error('Unknown action type:', action.type);
    }
  };

  return (
    <CssVarsProvider>
      <CssBaseline />
      <DispatcherContext.Provider value={dispatcher}>
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
