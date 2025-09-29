import {ReactNode, useEffect} from 'react';
import {CssVarsProvider, extendTheme, useColorScheme} from '@mui/joy/styles';
import CssBaseline from '@mui/joy/CssBaseline';
import {useAppState} from '../state/AppState';
import {colors} from '../theme';

const theme = extendTheme({
  colorSchemes: {
    light: {
      palette: {
        primary: {
          50: colors.primary.light,
          100: colors.primary.light,
          200: colors.primary.main,
          300: colors.primary.main,
          400: colors.primary.main,
          500: colors.primary.main, // main
          600: colors.primary.dark,
          700: colors.primary.dark,
          800: colors.primary.dark,
          900: colors.primary.dark,
        },
        neutral: {
          50: colors.neutral[0],
          100: colors.neutral[1],
          200: colors.neutral[2],
          300: colors.neutral[3],
          400: colors.neutral[4],
          500: colors.neutral[5],
          600: colors.neutral[6],
          700: colors.neutral[7],
          800: colors.neutral[8],
          900: colors.neutral[8],
        },
        background: {
          body: colors.background,
          surface: colors.neutral[0],
          level1: colors.neutral[1],
          level2: colors.neutral[2],
          level3: colors.neutral[3],
        },
        text: {
          primary: colors.neutral[7],
          secondary: colors.neutral[5],
          tertiary: colors.neutral[4],
        },
      },
    },
    dark: {
      palette: {
        primary: {
          50: colors.primary.light,
          100: colors.primary.light,
          200: colors.primary.main,
          300: colors.primary.main,
          400: colors.primary.main,
          500: colors.primary.main, // main
          600: colors.primary.dark,
          700: colors.primary.dark,
          800: colors.primary.dark,
          900: colors.primary.dark,
        },
        neutral: {
          50: colors.neutralDark[0],
          100: colors.neutralDark[1],
          200: colors.neutralDark[2],
          300: colors.neutralDark[3],
          400: colors.neutralDark[4],
          500: colors.neutralDark[5],
          600: colors.neutralDark[6],
          700: colors.neutralDark[7],
          800: colors.neutralDark[8],
          900: colors.neutralDark[8],
        },
        background: {
          body: colors.backgroundDark,
          surface: colors.neutralDark[1],
          level1: colors.neutralDark[2],
          level2: colors.neutralDark[3],
          level3: colors.neutralDark[4],
        },
        text: {
          primary: colors.neutralDark[8],
          secondary: colors.neutralDark[6],
          tertiary: colors.neutralDark[5],
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
