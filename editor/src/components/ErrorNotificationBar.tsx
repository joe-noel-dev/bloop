import React, {useEffect} from 'react';
import {Alert, Snackbar, IconButton} from '@mui/joy';
import {Close} from '@mui/icons-material';
import {useAppState} from '../state/AppState';
import {useDispatcher} from '../dispatcher/dispatcher';
import {hideErrorNotificationAction} from '../dispatcher/action';
import {spacing} from '../theme';

const AUTO_DISMISS_DELAY = 5000; // 5 seconds

export const ErrorNotificationBar: React.FC = () => {
  const {errorNotification} = useAppState();
  const dispatch = useDispatcher();

  useEffect(() => {
    if (!errorNotification) return;

    const timeout = setTimeout(() => {
      dispatch(hideErrorNotificationAction());
    }, AUTO_DISMISS_DELAY);

    return () => clearTimeout(timeout);
  }, [errorNotification, dispatch]);

  const handleClose = () => {
    dispatch(hideErrorNotificationAction());
  };

  return (
    <Snackbar
      open={!!errorNotification}
      onClose={handleClose}
      anchorOrigin={{
        vertical: 'top',
        horizontal: 'center',
      }}
      sx={{
        top: spacing.md,
        zIndex: 9999,
      }}
    >
      <Alert
        variant="soft"
        color="danger"
        sx={{
          minWidth: spacing.notificationMinWidth,
          maxWidth: spacing.notificationMaxWidth,
          display: 'flex',
          alignItems: 'center',
          gap: 1,
        }}
        endDecorator={
          <IconButton
            variant="soft"
            size="sm"
            color="danger"
            onClick={handleClose}
          >
            <Close />
          </IconButton>
        }
      >
        {errorNotification?.message}
      </Alert>
    </Snackbar>
  );
};
