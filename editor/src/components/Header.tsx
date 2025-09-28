import React, {useState, useContext} from 'react';
import {
  Box,
  IconButton,
  Menu,
  MenuItem,
  Typography,
  Divider,
} from '@mui/joy';
import {Person, Logout} from '@mui/icons-material';
import {BackendContext} from '../backend/Backend';
import {useDispatcher} from '../dispatcher/dispatcher';
import {signOutAction} from '../dispatcher/action';

export const Header = () => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const backend = useContext(BackendContext);
  const dispatch = useDispatcher();
  const user = backend?.getUser();

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleLogout = async () => {
    try {
      dispatch(signOutAction());
      handleClose();
      // Force page reload to reset app state
      window.location.reload();
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
      <IconButton
        onClick={handleClick}
        size="sm"
        variant="soft"
        color="neutral"
        sx={{
          borderRadius: 'sm',
        }}
      >
        <Person />
      </IconButton>

      <Menu
        anchorEl={anchorEl}
        open={Boolean(anchorEl)}
        onClose={handleClose}
        placement="bottom-end"
        sx={{
          minWidth: 200,
        }}
      >
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
    </Box>
  );
};
