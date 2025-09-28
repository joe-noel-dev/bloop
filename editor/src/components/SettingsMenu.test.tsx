import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { SettingsMenu } from './SettingsMenu';
import { AppStateContext } from '../state/AppState';
import { DispatcherContext } from '../dispatcher/dispatcher';
import { createThemeState } from '../state/ThemeState';
import { emptyProject } from '../api/project-helpers';

// Mock MUI icons
vi.mock('@mui/icons-material', () => ({
  Settings: () => <div>Settings Icon</div>,
  LightMode: () => <div>Light Mode Icon</div>,
  DarkMode: () => <div>Dark Mode Icon</div>,
  Brightness6: () => <div>System Icon</div>,
}));

const mockDispatch = vi.fn();

const mockAppState = {
  project: emptyProject(),
  projects: [],
  playing: false,
  saveState: 'idle' as const,
  sampleStates: new Map(),
  theme: createThemeState(),
};

const TestWrapper = ({ children }: { children: React.ReactNode }) => (
  <AppStateContext.Provider value={mockAppState}>
    <DispatcherContext.Provider value={mockDispatch}>
      {children}
    </DispatcherContext.Provider>
  </AppStateContext.Provider>
);

describe('SettingsMenu Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders settings button', () => {
    render(
      <TestWrapper>
        <SettingsMenu />
      </TestWrapper>
    );

    expect(screen.getByText('Settings Icon')).toBeInTheDocument();
  });

  it('shows settings menu when clicked', () => {
    render(
      <TestWrapper>
        <SettingsMenu />
      </TestWrapper>
    );

    const settingsButton = screen.getByText('Settings Icon').closest('button');
    expect(settingsButton).toBeInTheDocument();
    
    fireEvent.click(settingsButton!);

    expect(screen.getByText('Settings')).toBeInTheDocument();
    expect(screen.getByText('Theme')).toBeInTheDocument();
  });

  it('displays theme options', () => {
    render(
      <TestWrapper>
        <SettingsMenu />
      </TestWrapper>
    );

    const settingsButton = screen.getByText('Settings Icon').closest('button');
    fireEvent.click(settingsButton!);

    // Check that theme selector is present
    expect(screen.getByText('Theme')).toBeInTheDocument();
  });
});