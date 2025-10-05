import {render, screen} from '@testing-library/react';
import {describe, it, expect, vi, beforeEach} from 'vitest';
import {TransportBar} from '../src/components/TransportBar';
import {AppStateContext} from '../src/state/AppState';
import {DispatcherContext} from '../src/dispatcher/dispatcher';
import {
  createTestAppState,
  createTestTheme,
  createTestAppStateWithPlayback,
} from '../src/test-utils/app-state-helpers';
import Long from 'long';

// Mock Material-UI icons
vi.mock('@mui/icons-material', () => ({
  PlayArrow: () => <svg data-testid="play-icon" />,
  Stop: () => <svg data-testid="stop-icon" />,
}));

// Mock the progress service - simplified version
vi.mock('../src/audio/ProgressService', () => ({
  useProgressSubscription: vi.fn(() => {
    // Mock that doesn't interfere with component rendering
  }),
}));

// Mock the model hooks
vi.mock('../src/model-hooks/song-hooks', () => ({
  useSelectedSong: () => ({
    id: Long.fromNumber(1),
    name: 'Test Song',
    sections: [],
    tempo: 120,
    samples: [],
  }),
  useSong: () => ({
    id: Long.fromNumber(1),
    name: 'Playing Song',
    sections: [],
    tempo: 120,
    samples: [],
  }),
}));

vi.mock('../src/model-hooks/section-hooks', () => ({
  useSelectedSection: () => ({
    id: Long.fromNumber(1),
    name: 'Test Section',
    bars: 4,
    beatPattern: [],
  }),
  useSectionById: () => ({
    id: Long.fromNumber(1),
    name: 'Playing Section',
    bars: 4,
    beatPattern: [],
  }),
}));

describe('TransportBar - Progress Display', () => {
  const mockDispatch = vi.fn();

  beforeEach(() => {
    mockDispatch.mockClear();
  });

  const renderTransportBar = (appState = createTestAppState()) => {
    return render(
      <DispatcherContext.Provider value={mockDispatch}>
        <AppStateContext.Provider value={appState}>
          <TransportBar />
        </AppStateContext.Provider>
      </DispatcherContext.Provider>
    );
  };

  it('hides progress when not playing', () => {
    const appState = createTestAppState();
    renderTransportBar(appState);

    // Progress container should have opacity 0 when not playing
    const progressContainer =
      screen.getByText('Sec').closest('[data-testid]') ||
      screen.getByText('Sec').parentElement?.parentElement;

    if (progressContainer) {
      expect(progressContainer).toHaveStyle('opacity: 0');
    }
  });

  it('shows progress when playing', () => {
    const playingState = createTestAppStateWithPlayback(
      {
        songId: Long.fromNumber(1),
        sectionId: Long.fromNumber(1),
      },
      {
        theme: createTestTheme('light'),
      }
    );

    renderTransportBar(playingState);

    // Progress container should have opacity 1 when playing
    const progressContainer =
      screen.getByText('Sec').closest('[data-testid]') ||
      screen.getByText('Sec').parentElement?.parentElement;

    if (progressContainer) {
      expect(progressContainer).toHaveStyle('opacity: 1');
    }
  });

  it('displays section and song beat elements', () => {
    const playingState = createTestAppStateWithPlayback(
      {
        songId: Long.fromNumber(1),
        sectionId: Long.fromNumber(1),
      },
      {
        theme: createTestTheme('light'),
      }
    );

    renderTransportBar(playingState);

    // Should display section and song beat labels
    expect(screen.getByText('Sec')).toBeInTheDocument();
    expect(screen.getByText('Song')).toBeInTheDocument();
  });

  it('handles no playback state gracefully', () => {
    const appState = createTestAppState();
    renderTransportBar(appState);

    // Should render without errors and show default beat values
    expect(screen.getByText('Sec')).toBeInTheDocument();
    expect(screen.getByText('Song')).toBeInTheDocument();

    // Should show 0 for both beats when no progress
    const zeroTexts = screen.getAllByText('0');
    expect(zeroTexts).toHaveLength(2);
  });
});
