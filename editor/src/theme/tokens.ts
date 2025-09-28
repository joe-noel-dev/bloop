/**
 * Centralized theming tokens for the Bloop editor
 * These tokens provide consistent values across the application
 */

// Spacing tokens (based on 8px unit system)
export const spacing = {
  unit: 8,
  xs: 4,
  sm: 8,
  md: 16,
  lg: 24,
  xl: 32,
  xxl: 40,
  xxxl: 48,
  xxxxl: 56,
  xxxxxl: 64,
} as const;

// Color palette tokens
export const colors = {
  // Primary colors
  primary: {
    main: '#ffab91',
    light: '#ffddc1',
    dark: '#c97b63',
  },
  
  // Secondary colors  
  secondary: {
    main: '#1a237e',
    light: '#534bae',
    dark: '#000051',
  },

  // Neutral colors
  neutral: {
    0: 'white',
    1: 'hsl(240, 5%, 88.5%)',
    2: 'hsl(240, 5%, 75%)',
    3: 'hsl(240, 5%, 67.5%)',
    4: 'hsl(240, 5%, 50%)',
    5: 'hsl(240, 5%, 37.5%)',
    6: 'hsl(240, 5%, 25%)',
    7: 'hsl(240, 5%, 12.5%)',
    8: 'black',
  },

  // Palette colors
  palette: {
    1: '#bcd8c1',
    2: '#7f636e', 
    3: '#388697',
    4: '#cc2936',
    5: '#a93f55',
  },

  // Background
  background: 'white',
} as const;

// Shadow tokens
export const shadows = {
  level1: '0px 1px 3px rgba(0, 0, 0, 0.12), 0px 1px 2px rgba(0, 0, 0, 0.24)',
  level2: '0px 5px 10px rgba(0, 0, 0, 0.2), 0px 6px 6px rgba(0, 0, 0, 0.24)',
  level3: '0px 15px 30px rgba(0, 0, 0, 0.3), 0px 12px 12px rgba(0, 0, 0, 0.24)',
  
  // Common hover/interaction shadows
  hover: '0 4px 12px rgba(0, 0, 0, 0.15)',
  focus: '0 2px 8px rgba(0, 0, 0, 0.08)',
  active: '0 2px 4px rgba(0, 0, 0, 0.1)',
  elevated: '0 4px 12px rgba(0, 0, 0, 0.2)',
  soft: '0 2px 8px rgba(0, 0, 0, 0.15)',
} as const;

// Border radius tokens
export const radii = {
  none: 0,
  sm: 2,
  md: 4,
  lg: 8,
  full: 9999,
} as const;

// Text color tokens for different backgrounds
export const textColors = {
  onBackground: 'black',
  onPrimary: 'black',
  onPrimaryLight: 'black', 
  onPrimaryDark: 'black',
  onSecondary: 'white',
  onSecondaryLight: 'white',
  onSecondaryDark: 'white',
} as const;

// Transition tokens
export const transitions = {
  fast: 'all 0.2s ease',
  normal: 'all 0.3s ease',
  slow: 'all 0.5s ease',
  
  // Specific easing curves
  easeOut: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
  easeIn: 'all 0.3s cubic-bezier(0.4, 0, 1, 1)',
  bounce: 'all 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55)',
  
  // Specific property transitions
  transform: 'transform 0.1s ease',
} as const;

// Typography tokens (font families)
export const fonts = {
  body: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  mono: 'source-code-pro, Menlo, Monaco, Consolas, "Courier New", monospace',
} as const;

// Export type definitions for TypeScript
export type SpacingToken = keyof typeof spacing;
export type ColorToken = keyof typeof colors;
export type ShadowToken = keyof typeof shadows;
export type RadiusToken = keyof typeof radii;
export type TextColorToken = keyof typeof textColors;
export type TransitionToken = keyof typeof transitions;
export type FontToken = keyof typeof fonts;

// Utility functions for accessing tokens
export const getSpacing = (token: SpacingToken) => `${spacing[token]}px`;
export const getColor = (token: string) => {
  // Handle nested color tokens like 'primary.main', 'neutral.3'
  const parts = token.split('.');
  if (parts.length === 2) {
    const [group, variant] = parts;
    return (colors as any)[group]?.[variant] || token;
  }
  return (colors as any)[token] || token;
};
export const getShadow = (token: ShadowToken) => shadows[token];
export const getRadius = (token: RadiusToken) => `${radii[token]}px`;
export const getTransition = (token: TransitionToken) => transitions[token];