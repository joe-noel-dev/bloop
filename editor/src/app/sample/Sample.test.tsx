import {describe, it, expect, vi} from 'vitest';
import {render, screen} from '@testing-library/react';
import {Sample} from './Sample';
import {AppStateContext} from '../../state/AppState';
import {DispatcherContext} from '../../dispatcher/dispatcher';
import {AudioControllerContext} from '../../audio/AudioControllerContext';
import {emptyProject} from '../../api/project-helpers';
import {createTestAppStateWithSamples} from '../../test-utils/app-state-helpers';
import Long from 'long';

// Mock Material-UI icons
vi.mock('@mui/icons-material', () => ({
  Delete: () => <svg data-testid="delete-icon" />,
  FileUpload: () => <svg data-testid="file-upload-icon" />,
  Download: () => <svg data-testid="download-icon" />,
  Sync: () => <svg data-testid="sync-icon" />,
  Error: () => <svg data-testid="error-icon" />,
}));

// Mock dependencies
const mockDispatch = () => {};
const mockAudioController = {
  setProject: () => {},
  setProjectInfo: () => {},
  play: () => {},
  stop: () => {},
  setPlaybackStateChangeCallback: () => {},
  setDispatch: () => {},
  getSampleState: () => null,
};

const testSampleId = Long.fromNumber(123);
const testSongId = Long.fromNumber(456);

// Create a project with a song for testing
const createProjectWithSong = () => {
  const project = emptyProject();
  project.songs = [
    {
      id: testSongId,
      name: 'Test Song',
      tempo: {bpm: 120},
      sections: [],
      sample: undefined,
    },
  ];
  return project;
};

const TestWrapper = ({
  children,
  sampleStates = new Map(),
}: {
  children: React.ReactNode;
  sampleStates?: Map<Long, any>;
}) => (
  <AppStateContext.Provider
    value={createTestAppStateWithSamples(sampleStates, {
      project: createProjectWithSong(),
    })}
  >
    <DispatcherContext.Provider value={mockDispatch}>
      <AudioControllerContext.Provider value={mockAudioController}>
        {children}
      </AudioControllerContext.Provider>
    </DispatcherContext.Provider>
  </AppStateContext.Provider>
);

describe('Sample Component', () => {
  it('shows upload button when no sample and no loading state', () => {
    render(
      <TestWrapper>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Upload Sample')).toBeInTheDocument();
  });

  it('shows download status chip and upload button when sample is loading', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, {state: 'loading'});

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Downloading...')).toBeInTheDocument();
    expect(screen.getByText('Upload Sample')).toBeInTheDocument();
  });

  it('shows download status chip and upload button when sample is converting', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, {state: 'converting'});

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Converting...')).toBeInTheDocument();
    expect(screen.getByText('Upload Sample')).toBeInTheDocument();
  });

  it('shows download error chip and upload button when sample loading fails', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, {state: 'error'});

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Download Error')).toBeInTheDocument();
    expect(screen.getByText('Upload Sample')).toBeInTheDocument();
  });

  it('shows remove button when sample is loaded and no download status', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, {state: 'loaded'});

    // Update the test wrapper to include a sample in the project
    const projectWithSample = createProjectWithSong();
    projectWithSample.songs[0].sample = {
      id: testSampleId,
      name: 'test-sample.wav',
      tempo: {bpm: 120},
      sampleRate: 44100,
      sampleCount: Long.fromNumber(1000),
      channelCount: 2,
    };

    const appStateWithSample = createTestAppStateWithSamples(sampleStates, {
      project: projectWithSample,
    });

    render(
      <AppStateContext.Provider value={appStateWithSample}>
        <DispatcherContext.Provider value={mockDispatch}>
          <AudioControllerContext.Provider value={mockAudioController}>
            <Sample sampleId={testSampleId} songId={testSongId} />
          </AudioControllerContext.Provider>
        </DispatcherContext.Provider>
      </AppStateContext.Provider>
    );

    expect(screen.getByText('Remove Sample')).toBeInTheDocument();
    expect(screen.queryByText('Downloading...')).not.toBeInTheDocument();
    expect(screen.queryByText('Converting...')).not.toBeInTheDocument();
    expect(screen.queryByText('Download Error')).not.toBeInTheDocument();
  });
});
