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
import {
  Action,
  ADD_SECTION,
  ADD_SONG,
  AddSectionAction,
  MOVE_SONG,
  MoveSongAction,
  SELECT_SONG,
  SelectSongAction,
  UPDATE_SECTION,
  UPDATE_SONG,
  UpdateSectionAction,
  UpdateSongAction,
} from './dispatcher/action';
import {
  addSection,
  addSong,
  moveSong,
  selectSong,
  updateSection,
  updateSong,
} from './api/project-helpers';

const App = () => {
  const [backend, setBackend] = useState<Backend | null>(null);
  const [state, setState] = useState<AppState>({
    project: undefined,
    projectInfo: null,
    projects: [],
  });

  console.dir(state.project);

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

    const newProject = {...state.project};

    switch (action.type) {
      case ADD_SONG:
        addSong(newProject);
        break;

      case ADD_SECTION:
        const addSectionAction = action as AddSectionAction;
        addSection(newProject, addSectionAction.songId);
        break;

      case SELECT_SONG:
        const selectSongAction = action as SelectSongAction;
        selectSong(newProject, selectSongAction.songId);
        break;

      case MOVE_SONG:
        const moveSongAction = action as MoveSongAction;
        moveSong(newProject, moveSongAction.fromIndex, moveSongAction.toIndex);
        break;

      case UPDATE_SECTION:
        const updateSectionAction = action as UpdateSectionAction;
        updateSection(
          newProject,
          updateSectionAction.songId,
          updateSectionAction.newSection
        );
        break;

      case UPDATE_SONG:
        const updateSongAction = action as UpdateSongAction;
        updateSong(newProject, updateSongAction.newSong);
        break;

      default:
        console.error('Unknown action type:', action.type);
    }

    setState({
      ...state,
      project: newProject,
    });
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
