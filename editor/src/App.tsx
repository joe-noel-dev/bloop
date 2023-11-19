import {useEffect, useState} from 'react';
import {CoreContext} from './core/use-core';
import {Core, createCore} from './core/Core';
import {PlaybackState} from './model/playback-state';
import {Progress} from './model/progress';
import {ProjectInfo} from './model/project-info';
import {WaveformData} from './model/waveform';
import {CoreDataContext} from './core/CoreData';
import CssBaseline from '@mui/joy/CssBaseline';
import {CssVarsProvider} from '@mui/joy/styles';
import {Box, Divider} from '@mui/joy';
import {Project as ModelProject} from './model';
import '@fontsource/inter';
import {Connection} from './app/Connection';
import {Project} from './app/project/Project';

const App = () => {
  const [core, setCore] = useState<Core | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [project, setProject] = useState<ModelProject>();
  const [playbackState, setPlaybackState] = useState<PlaybackState>();
  const [progress, setProgress] = useState<Progress>();
  const [projects, setProjects] = useState<ProjectInfo[]>([]);
  const [waveforms, setWaveforms] = useState(new Map<string, WaveformData>());

  useEffect(() => {
    if (core) {
      core.disconnect();
    }

    const newCore = createCore({
      onConnected: () => setIsConnected(true),
      onDisconnected: () => setIsConnected(false),
      onProject: setProject,
      onPlaybackState: setPlaybackState,
      onProgress: setProgress,
      onProjects: setProjects,
      onWaveform: setWaveforms,
    });

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
            progress,
            projects,
            waveforms,
          }}
        >
          <Box
            sx={{display: 'flex', flexDirection: 'column', minHeight: '100vh'}}
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
