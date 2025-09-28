import { useAppState } from './AppState';
import { useDispatcher } from '../dispatcher/dispatcher';
import { setThemeModeAction } from '../dispatcher/action';
import { ThemeMode } from './ThemeState';

export const useTheme = () => {
  const { theme } = useAppState();
  const dispatch = useDispatcher();

  const setThemeMode = (mode: ThemeMode) => {
    dispatch(setThemeModeAction(mode));
  };

  return {
    mode: theme.mode,
    effectiveMode: theme.effectiveMode,
    setThemeMode,
  };
};