import {
  Box,
  Dropdown,
  Menu,
  MenuButton,
  MenuItem,
  Typography,
  IconButton,
} from '@mui/joy';
import {
  Settings,
  LightMode,
  DarkMode,
  Brightness6,
  Check,
} from '@mui/icons-material';
import {useTheme} from '../state/useTheme';
import {ThemeMode} from '../state/ThemeState';

const themeOptions = [
  {value: 'light' as const, label: 'Light', icon: <LightMode />},
  {value: 'dark' as const, label: 'Dark', icon: <DarkMode />},
  {value: 'system' as const, label: 'System', icon: <Brightness6 />},
];

export const SettingsMenu = () => {
  const {mode, setThemeMode} = useTheme();

  const handleThemeSelect = (selectedMode: ThemeMode) => {
    setThemeMode(selectedMode);
  };

  return (
    <Dropdown>
      <MenuButton
        slots={{root: IconButton}}
        slotProps={{
          root: {
            size: 'sm',
            variant: 'soft',
            color: 'neutral',
            sx: {
              borderRadius: 'sm',
            },
          },
        }}
      >
        <Settings />
      </MenuButton>
      <Menu placement="bottom-end" sx={{minWidth: 200}}>
        <Box sx={{p: 1.5}}>
          <Typography level="title-sm" sx={{mb: 1}}>
            Settings
          </Typography>

          <Box sx={{mb: 1}}>
            <Typography level="body-sm" sx={{mb: 1, color: 'neutral.500'}}>
              Theme
            </Typography>
            {themeOptions.map((option) => (
              <MenuItem
                key={option.value}
                onClick={() => handleThemeSelect(option.value)}
                sx={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 1,
                  py: 0.5,
                  px: 1,
                  borderRadius: 'sm',
                  bgcolor:
                    mode === option.value ? 'neutral.100' : 'transparent',
                }}
              >
                {option.icon}
                <Typography level="body-sm">{option.label}</Typography>
                {mode === option.value && (
                  <Check sx={{ml: 'auto', fontSize: '1rem'}} />
                )}
              </MenuItem>
            ))}
          </Box>
        </Box>
      </Menu>
    </Dropdown>
  );
};
