import {ReactNode, useEffect} from 'react';
import {CssVarsProvider, extendTheme, useColorScheme} from '@mui/joy/styles';
import CssBaseline from '@mui/joy/CssBaseline';
import {useAppState} from '../state/AppState';

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
        background: {
          body: 'white',
          surface: 'hsl(240, 5%, 98%)',
          level1: 'hsl(240, 5%, 95%)',
          level2: 'hsl(240, 5%, 92%)',
          level3: 'hsl(240, 5%, 88%)',
        },
        text: {
          primary: 'hsl(240, 5%, 12%)',
          secondary: 'hsl(240, 5%, 37%)',
          tertiary: 'hsl(240, 5%, 60%)',
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
        background: {
          body: 'hsl(240, 5%, 8%)',
          surface: 'hsl(240, 5%, 12%)',
          level1: 'hsl(240, 5%, 16%)',
          level2: 'hsl(240, 5%, 20%)',
          level3: 'hsl(240, 5%, 24%)',
        },
        text: {
          primary: 'white',
          secondary: 'hsl(240, 5%, 75%)',
          tertiary: 'hsl(240, 5%, 50%)',
        },
      },
    },
  },
});

interface ThemeWrapperProps {
  children: ReactNode;
}

// Component to sync MUI color scheme with our app state
const ThemeSync = () => {
  const {theme: themeState} = useAppState();
  const {setMode} = useColorScheme();

  useEffect(() => {
    setMode(themeState.effectiveMode);
  }, [themeState.effectiveMode, setMode]);

  return null;
};

export const ThemeWrapper = ({children}: ThemeWrapperProps) => {
  const {theme: themeState} = useAppState();

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
      disableTransitionOnChange={false}
    >
      <CssBaseline />
      <ThemeSync />
      {children}
    </CssVarsProvider>
  );
};
