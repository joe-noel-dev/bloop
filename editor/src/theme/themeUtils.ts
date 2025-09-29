import {ThemeMode} from '../state/ThemeState';
import {colors, textColors} from './tokens';

export const getThemeColors = (mode: ThemeMode) => {
  if (mode === 'dark') {
    return {
      ...colors,
      neutral: colors.neutralDark,
      background: colors.backgroundDark,
    };
  }
  return colors;
};

export const getThemeTextColors = (mode: ThemeMode) => {
  if (mode === 'dark') {
    return {
      ...textColors,
      onBackground: textColors.onBackgroundDark,
    };
  }
  return textColors;
};

export const getThemeColor = (token: string, mode: ThemeMode = 'light') => {
  const themeColors = getThemeColors(mode);

  // Handle nested color tokens like 'primary.main', 'neutral.3'
  const parts = token.split('.');
  if (parts.length === 2) {
    const [group, variant] = parts;
    return (themeColors as any)[group]?.[variant] || token;
  }
  return (themeColors as any)[token] || token;
};
