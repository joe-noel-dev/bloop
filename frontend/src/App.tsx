import {useEffect, useState} from 'react';
import {Header} from './features/header/Header';
import {Songs} from './features/songs/Songs';
import {Transport} from './features/transport/Transport';
import {NoConnectionOverlay} from './features/connection/NoConnectionOverlay';
import {CoreContext} from './features/core/use-core';
import {Core, createCore} from './features/core/Core';
import {Project} from './model/project';
import {PlaybackState} from './model/playback-state';
import {Progress} from './model/progress';
import {ProjectInfo} from './model/project-info';
import {WaveformData} from './model/waveform';
import {CoreDataContext} from './features/core/CoreData';
import styles from './App.module.css';

const App = () => {
  const [core, setCore] = useState<Core | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [project, setProject] = useState<Project>();
  const [playbackState, setPlaybackState] = useState<PlaybackState>();
  const [progress, setProgress] = useState<Progress>();
  const [projects, setProjects] = useState<ProjectInfo[]>([]);
  const [waveforms, setWaveforms] = useState(new Map<string, WaveformData>());

  useEffect(() => {
    const core = createCore({
      onConnected: () => setIsConnected(true),
      onDisconnected: () => setIsConnected(false),
      onProject: setProject,
      onPlaybackState: setPlaybackState,
      onProgress: setProgress,
      onProjects: setProjects,
      onWaveform: setWaveforms,
    });

    setCore(core);
  }, []);

  if (!core) {
    return <></>;
  }

  return (
    <>
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
          {!isConnected && (
            <div className={styles.container}>
              <NoConnectionOverlay />
            </div>
          )}
          {isConnected && (
            <div className={styles.container}>
              <Header />
              <Songs />
              <Transport />
            </div>
          )}
        </CoreDataContext.Provider>
      </CoreContext.Provider>
    </>
  );
};

export default App;
