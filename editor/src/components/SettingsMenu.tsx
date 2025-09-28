import {
  Box,
  Dropdown,
  Menu,
  MenuButton,
  Typography,
  IconButton,
  Option,
  Select,
} from '@mui/joy';
import { Settings, LightMode, DarkMode, Brightness6 } from '@mui/icons-material';
import { useTheme } from '../state/useTheme';
import { ThemeMode } from '../state/ThemeState';

const themeOptions = [
  { value: 'light' as const, label: 'Light', icon: <LightMode /> },
  { value: 'dark' as const, label: 'Dark', icon: <DarkMode /> },
  { value: 'system' as const, label: 'System', icon: <Brightness6 /> },
];

export const SettingsMenu = () => {
  const { mode, setThemeMode } = useTheme();

  const handleThemeChange = (_: any, newValue: ThemeMode | null) => {
    if (newValue) {
      setThemeMode(newValue);
    }
  };

  return (
    <Dropdown>
      <MenuButton
        slots={{ root: IconButton }}
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
      <Menu placement="bottom-end" sx={{ minWidth: 200 }}>
        <Box sx={{ p: 1.5 }}>
          <Typography level="title-sm" sx={{ mb: 1 }}>
            Settings
          </Typography>
          
          <Box sx={{ mb: 2 }}>
            <Typography level="body-sm" sx={{ mb: 1, color: 'neutral.500' }}>
              Theme
            </Typography>
            <Select
              value={mode}
              onChange={handleThemeChange}
              size="sm"
              sx={{ minWidth: 120 }}
            >
              {themeOptions.map((option) => (
                <Option key={option.value} value={option.value}>
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    {option.icon}
                    {option.label}
                  </Box>
                </Option>
              ))}
            </Select>
          </Box>
        </Box>
      </Menu>
    </Dropdown>
  );
};