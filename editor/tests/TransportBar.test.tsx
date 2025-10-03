import {render, screen, fireEvent} from '@testing-library/react';
import {describe, it, expect, vi, beforeEach} from 'vitest';
import {TransportBar} from '../src/components/TransportBar';
import {AppStateContext} from '../src/state/AppState';
import {DispatcherContext} from '../src/dispatcher/dispatcher';
import {playAction, stopAction} from '../src/dispatcher/action';
import {emptyProject} from '../src/api/project-helpers';
import Long from 'long';

// Mock Material-UI icons
vi.mock('@mui/icons-material', () => ({
  PlayArrow: () => <svg data-testid="play-icon" />,
  Stop: () => <svg data-testid="stop-icon" />,
}));

// Mock the model hooks
vi.mock('../src/model-hooks/song-hooks', () => ({
  useSelectedSong: () => ({
    id: Long.fromNumber(1),
    name: 'Test Song',
    sections: [],
  }),
  useSong: () => ({
    id: Long.fromNumber(1),
    name: 'Playing Song',
    sections: [],
  }),
}));

vi.mock('../src/model-hooks/section-hooks', () => ({
  useSelectedSection: () => ({
    id: Long.fromNumber(1),
    name: 'Test Section',
  }),
  useSectionById: () => ({
    id: Long.fromNumber(1),
    name: 'Playing Section',
  }),
}));

describe('TransportBar', () => {
  const mockDispatch = vi.fn();

  const mockAppState = {
    project: emptyProject(),
    projectInfo: null,
    projects: [],
    playing: false,
    saveState: 'idle' as const,
    sampleStates: new Map(),
    theme: {
      mode: 'light' as const,
      effectiveMode: 'light' as const,
    },
  };

  beforeEach(() => {
    mockDispatch.mockClear();
  });

  const renderTransportBar = (appState = mockAppState) => {
    return render(
      <DispatcherContext.Provider value={mockDispatch}>
        <AppStateContext.Provider value={appState}>
          <TransportBar />
        </AppStateContext.Provider>
      </DispatcherContext.Provider>
    );
  };

  it('renders with song and section names', () => {
    renderTransportBar();

    expect(screen.getByText('Test Song')).toBeInTheDocument();
    expect(screen.getByText('Test Section')).toBeInTheDocument();
  });

  it('shows play button when not playing', () => {
    renderTransportBar();

    const button = screen.getByRole('button');
    expect(button).toBeInTheDocument();
    // The play icon should be present
    expect(screen.getByTestId('play-icon')).toBeInTheDocument();
  });

  it('shows stop button when playing', () => {
    const playingState = {
      ...mockAppState,
      playing: true,
      playingSongId: Long.fromNumber(1),
      playingSectionId: Long.fromNumber(1),
    };

    renderTransportBar(playingState);

    const button = screen.getByRole('button');
    expect(button).toBeInTheDocument();
    // The stop icon should be present
    expect(screen.getByTestId('stop-icon')).toBeInTheDocument();
  });

  it('dispatches play action when play button is clicked', () => {
    renderTransportBar();

    const button = screen.getByRole('button');
    fireEvent.click(button);

    expect(mockDispatch).toHaveBeenCalledWith(
      playAction(Long.fromNumber(1), Long.fromNumber(1))
    );
  });

  it('dispatches stop action when stop button is clicked while playing', () => {
    const playingState = {
      ...mockAppState,
      playing: true,
      playingSongId: Long.fromNumber(1),
      playingSectionId: Long.fromNumber(1),
    };

    renderTransportBar(playingState);

    const button = screen.getByRole('button');
    fireEvent.click(button);

    expect(mockDispatch).toHaveBeenCalledWith(stopAction());
  });

  it('shows playing song and section when playing', () => {
    const playingState = {
      ...mockAppState,
      playing: true,
      playingSongId: Long.fromNumber(1),
      playingSectionId: Long.fromNumber(1),
    };

    renderTransportBar(playingState);

    expect(screen.getByText('Playing Song')).toBeInTheDocument();
    expect(screen.getByText('Playing Section')).toBeInTheDocument();
  });
});
