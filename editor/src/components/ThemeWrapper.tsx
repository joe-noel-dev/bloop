import { ReactNode, useEffect } from 'react';
import { CssVarsProvider, extendTheme } from '@mui/joy/styles';
import CssBaseline from '@mui/joy/CssBaseline';
import { useAppState } from '../state/AppState';

const theme = extendTheme({
  colorSchemes: {
    light: {
      palette: {
        primary: {
          50: '#ffddc1',
          100: '#ffddc1', 
          200: '#ffab91',
          300: '#ffab91',
          400: '#ffab91',
          500: '#ffab91', // main
          600: '#c97b63',
          700: '#c97b63',
          800: '#c97b63',
          900: '#c97b63',
        },
        neutral: {
          50: 'white',
          100: 'hsl(240, 5%, 88.5%)',
          200: 'hsl(240, 5%, 75%)',
          300: 'hsl(240, 5%, 67.5%)',
          400: 'hsl(240, 5%, 50%)',
          500: 'hsl(240, 5%, 37.5%)',
          600: 'hsl(240, 5%, 25%)',
          700: 'hsl(240, 5%, 12.5%)',
          800: 'black',
          900: 'black',
        },
      },
    },
    dark: {
      palette: {
        primary: {
          50: '#ffddc1',
          100: '#ffddc1', 
          200: '#ffab91',
          300: '#ffab91',
          400: '#ffab91',
          500: '#ffab91', // main
          600: '#c97b63',
          700: '#c97b63',
          800: '#c97b63',
          900: '#c97b63',
        },
        neutral: {
          50: 'black',
          100: 'hsl(240, 5%, 12.5%)',
          200: 'hsl(240, 5%, 25%)',
          300: 'hsl(240, 5%, 37.5%)',
          400: 'hsl(240, 5%, 50%)',
          500: 'hsl(240, 5%, 67.5%)',
          600: 'hsl(240, 5%, 75%)',
          700: 'hsl(240, 5%, 88.5%)',
          800: 'white',
          900: 'white',
        },
      },
    },
  },
});

interface ThemeWrapperProps {
  children: ReactNode;
}

export const ThemeWrapper = ({ children }: ThemeWrapperProps) => {
  const { theme: themeState } = useAppState();

  // Apply theme to document root for CSS variables
  useEffect(() => {
    const root = document.documentElement;
    
    if (themeState.effectiveMode === 'dark') {
      root.setAttribute('data-theme', 'dark');
    } else {
      root.removeAttribute('data-theme');
    }
  }, [themeState.effectiveMode]);

  return (
    <CssVarsProvider 
      theme={theme} 
      defaultMode={themeState.effectiveMode}
      modeStorageKey="joy-mode"
    >
      <CssBaseline />
      {children}
    </CssVarsProvider>
  );
};