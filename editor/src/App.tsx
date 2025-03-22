import {useEffect, useState} from 'react';
import {CoreContext} from './core/use-core';
import {Core, createCore} from './core/Core';
import {CoreDataContext} from './core/CoreData';
import CssBaseline from '@mui/joy/CssBaseline';
import {CssVarsProvider} from '@mui/joy/styles';
import {Box, Divider} from '@mui/joy';
import {
  Project as ModelProject,
  PlaybackState,
  Progress,
  ProjectInfo,
  WaveformData,
} from './api/bloop';
import '@fontsource/inter';
import {Connection} from './app/Connection';
import {Project} from './app/project/Project';
import {ID} from './api/helpers';

const App = () => {
  const [core, setCore] = useState<Core | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [project, setProject] = useState<ModelProject>();
  const [playbackState, setPlaybackState] = useState<PlaybackState>();
  const [projects, setProjects] = useState<ProjectInfo[]>([]);
  const [waveforms, setWaveforms] = useState(new Map<ID, WaveformData>());
  const [progress, setProgress] = useState<Progress>({
    songProgress: 0,
    sectionProgress: 0,
    sectionBeat: 0,
  });

  useEffect(() => {
    if (core) {
      core.disconnect();
    }

    const newCore = createCore();
    newCore.events.on('connect', () => setIsConnected(true));
    newCore.events.on('disconnect', () => setIsConnected(false));
    newCore.events.on('project', setProject);
    newCore.events.on('playback-state', setPlaybackState);
    newCore.events.on('projects', setProjects);
    newCore.events.on('waveforms', setWaveforms);
    newCore.events.on('progress', setProgress);

    setCore(newCore);
  }, []);

  if (!core) {
    return <></>;
  }

  return (
    <CssVarsProvider>
      <CssBaseline />

      <CoreContext.Provider value={core}>
        <CoreDataContext.Provider
          value={{
            project,
            playbackState,
            projects,
            progress,
            waveforms,
          }}
        >
          <Box
            sx={{
              display: 'flex',
              flexDirection: 'column',
              minHeight: '100vh',
            }}
          >
            <Box>
              <Connection
                isConnected={isConnected}
                connect={core.connect}
                disconnect={core.disconnect}
              />
            </Box>
            <Divider />
            <Box sx={{flexGrow: 1}}>{isConnected && <Project />}</Box>
          </Box>
        </CoreDataContext.Provider>
      </CoreContext.Provider>
    </CssVarsProvider>
  );
};

export default App;
