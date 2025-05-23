import {useEffect, useState} from 'react';
import {CoreDataContext} from './core/CoreData';
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

const App = () => {
  const [backend, setBackend] = useState<Backend | null>(null);
  const [project, setProject] = useState<ModelProject>();
  const [projects, setProjects] = useState<DbProject[]>([]);
  const [projectInfo, setProjectInfo] = useState<DbProject | null>(null);

  const user = backend?.getUser();

  useEffect(() => {
    const newBackend = createBackend();
    newBackend.fetchProjects();
    newBackend.events.on('user', (user: DbUser | null) => {
      if (user) {
        newBackend.fetchProjects();
      }
    });
    newBackend.events.on('project', setProject);
    newBackend.events.on('projects', setProjects);
    newBackend.events.on('project-info', setProjectInfo);
    setBackend(newBackend);
  }, []);

  if (!backend) {
    return;
  }

  const dispatcher = (action: Action) => {
    switch (action.type) {
      case ADD_SONG:
        // FIXME: implement add song
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
          <CoreDataContext.Provider
            value={{
              project,
              projectInfo,
              projects,
            }}
          >
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
          </CoreDataContext.Provider>
        </BackendContext.Provider>
      </DispatcherContext.Provider>
    </CssVarsProvider>
  );
};

export default App;
