import {createContext, useContext} from 'react';

export type ThemeMode = 'light' | 'dark' | 'system';

export interface ThemeState {
  mode: ThemeMode;
  effectiveMode: 'light' | 'dark'; // The actual theme being used
}

export const createThemeState = (): ThemeState => {
  // Check localStorage first, then system preference
  let savedMode: ThemeMode | null = null;
  try {
    savedMode = localStorage.getItem('theme-mode') as ThemeMode;
  } catch (e) {
    // Fallback for environments without localStorage
    savedMode = null;
  }
  
  // Safely check system preference (fallback for test environments)
  let systemPrefersDark = false;
  try {
    systemPrefersDark = window.matchMedia?.('(prefers-color-scheme: dark)').matches || false;
  } catch (e) {
    // Fallback for environments without matchMedia
    systemPrefersDark = false;
  }
  
  const mode = savedMode || 'system';
  const effectiveMode = mode === 'system' 
    ? (systemPrefersDark ? 'dark' : 'light')
    : mode;

  return {
    mode,
    effectiveMode,
  };
};

export const ThemeStateContext = createContext<ThemeState>(createThemeState());

export const useThemeState = () => useContext(ThemeStateContext);