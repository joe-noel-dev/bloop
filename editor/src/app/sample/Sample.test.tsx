import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Sample } from './Sample';
import { AppStateContext } from '../../state/AppState';
import { DispatcherContext } from '../../dispatcher/dispatcher';
import { AudioControllerContext } from '../../audio/AudioControllerContext';
import { emptyProject } from '../../api/project-helpers';
import Long from 'long';

// Mock MUI icons
vi.mock('@mui/icons-material', () => ({
  Delete: () => <div>Delete Icon</div>,
  FileUpload: () => <div>FileUpload Icon</div>,
  Download: () => <div>Download Icon</div>,
  Sync: () => <div>Sync Icon</div>,
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
  project.songs = [{
    id: testSongId,
    name: 'Test Song',
    tempo: { bpm: 120 },
    sections: [],
    sample: undefined,
  }];
  return project;
};

const mockAppState = {
  project: createProjectWithSong(),
  projects: [],
  playing: false,
  saveState: 'idle' as const,
  sampleStates: new Map(),
};

const TestWrapper = ({ 
  children, 
  sampleStates = new Map() 
}: { 
  children: React.ReactNode;
  sampleStates?: Map<Long, any>;
}) => (
  <AppStateContext.Provider value={{ ...mockAppState, sampleStates }}>
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

  it('shows downloading indicator when sample is loading', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, { state: 'loading' });

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Downloading...')).toBeInTheDocument();
    expect(screen.getByText('Downloading...')).toBeDisabled();
  });

  it('shows converting indicator when sample is converting', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, { state: 'converting' });

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Converting...')).toBeInTheDocument();
    expect(screen.getByText('Converting...')).toBeDisabled();
  });

  it('shows error indicator when sample loading fails', () => {
    const sampleStates = new Map();
    sampleStates.set(testSampleId, { state: 'error' });

    render(
      <TestWrapper sampleStates={sampleStates}>
        <Sample sampleId={testSampleId} songId={testSongId} />
      </TestWrapper>
    );

    expect(screen.getByText('Error Loading Sample')).toBeInTheDocument();
    expect(screen.getByText('Error Loading Sample')).toBeDisabled();
  });
});