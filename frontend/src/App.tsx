import React, {useEffect, useState} from 'react';
import {Header} from './features/header/Header';
import {Songs} from './features/songs/Songs';
import {Transport} from './features/transport/Transport';
import styled from 'styled-components';
import {NoConnectionOverlay} from './features/connection/NoConnectionOverlay';
import {CoreContext} from './features/core/use-core';
import {Core, createCore} from './features/core/Core';
import {Project} from './model/project';
import {PlaybackState} from './model/playback-state';
import {Progress} from './model/progress';
import {ProjectInfo} from './model/project-info';
import {WaveformData} from './model/waveform';
import {CoreDataContext} from './features/core/CoreData';

const Container = styled.div`
  min-height: 100vh;
  max-height: 100vh;

  display: grid;
  grid-template-rows: auto minmax(0, 1fr) auto;
`;

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
            <Container>
              <NoConnectionOverlay />
            </Container>
          )}
          {isConnected && (
            <Container>
              <Header />
              <Songs />
              <Transport />
            </Container>
          )}
        </CoreDataContext.Provider>
      </CoreContext.Provider>
    </>
  );
};

export default App;
