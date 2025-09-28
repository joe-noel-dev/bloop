import {useContext} from 'react';
import {
  Box,
  Dropdown,
  Menu,
  MenuButton,
  MenuItem,
  Typography,
  Divider,
  IconButton,
} from '@mui/joy';
import {Person, Logout} from '@mui/icons-material';
import {BackendContext} from '../backend/Backend';
import {useDispatcher} from '../dispatcher/dispatcher';
import {signOutAction} from '../dispatcher/action';

export const Header = () => {
  const backend = useContext(BackendContext);
  const dispatch = useDispatcher();
  const user = backend?.getUser();

  const handleLogout = async () => {
    try {
      dispatch(signOutAction());
    } catch (error) {
      console.error('Error signing out:', error);
    }
  };

  if (!user) {
    return null;
  }

  return (
    <Box
      sx={{
        display: 'flex',
        justifyContent: 'flex-end',
        alignItems: 'center',
        padding: 2,
        borderBottom: '1px solid',
        borderColor: 'neutral.200',
        backgroundColor: 'background.surface',
      }}
    >
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
          <Person />
        </MenuButton>
        <Menu placement="bottom-end" sx={{minWidth: 200}}>
          <Box sx={{p: 1.5}}>
            <Typography level="body-sm" color="neutral">
              Signed in as
            </Typography>
            <Typography level="title-sm" sx={{fontWeight: 'bold'}}>
              {user.name}
            </Typography>
            <Typography level="body-xs" color="neutral">
              {user.email}
            </Typography>
          </Box>
          <Divider />
          <MenuItem onClick={handleLogout} color="danger">
            <Logout sx={{mr: 1}} />
            Sign Out
          </MenuItem>
        </Menu>
      </Dropdown>
    </Box>
  );
};
