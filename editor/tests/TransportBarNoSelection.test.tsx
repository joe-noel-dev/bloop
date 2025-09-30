import {render, screen} from '@testing-library/react';
import {describe, it, expect, vi} from 'vitest';
import {TransportBar} from '../src/components/TransportBar';
import {AppStateContext} from '../src/state/AppState';
import {DispatcherContext} from '../src/dispatcher/dispatcher';
import {emptyProject} from '../src/api/project-helpers';

// Mock Material-UI icons
vi.mock('@mui/icons-material', () => ({
  PlayArrow: () => <svg data-testid="play-icon" />,
  Stop: () => <svg data-testid="stop-icon" />,
}));

// Mock the model hooks to return undefined (no selection)
vi.mock('../src/model-hooks/song-hooks', () => ({
  useSelectedSong: () => undefined,
  useSong: () => undefined,
}));

vi.mock('../src/model-hooks/section-hooks', () => ({
  useSelectedSection: () => undefined,
  useSectionById: () => undefined,
}));

describe('TransportBar - No Selection', () => {
  const mockDispatch = vi.fn();

  const mockAppState = {
    project: emptyProject(),
    projects: [],
    playing: false,
    saveState: 'idle' as const,
    sampleStates: new Map(),
    theme: {
      mode: 'light' as const,
      effectiveMode: 'light' as const,
    },
  };

  const renderTransportBar = (appState = mockAppState) => {
    return render(
      <DispatcherContext.Provider value={mockDispatch}>
        <AppStateContext.Provider value={appState}>
          <TransportBar />
        </AppStateContext.Provider>
      </DispatcherContext.Provider>
    );
  };

  it('shows fallback text when no song/section selected', () => {
    renderTransportBar();

    expect(screen.getByText('No song selected')).toBeInTheDocument();
    expect(screen.getByText('No section selected')).toBeInTheDocument();
  });

  it('disables play button when no song/section selected', () => {
    renderTransportBar();

    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });
});
