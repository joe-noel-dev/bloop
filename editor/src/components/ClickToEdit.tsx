import {Box, Input, Typography} from '@mui/joy';
import {useState} from 'react';

interface Props {
  initialValue: string;
  onSave: (value: string) => void;
}

export const ClickToEdit = ({initialValue, onSave}: Props) => {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(initialValue);

  const submit = () => {
    onSave(value);
    setEditing(false);
  };

  const commonStyles = {
    padding: '4px',
  };

  return (
    <Box sx={{width: '100%'}}>
      {editing ? (
        <Input
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onBlur={submit}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              submit();
            }

            if (e.key === 'Escape') {
              setEditing(false);
            }
          }}
          autoFocus
          sx={commonStyles}
        />
      ) : (
        <Typography onClick={() => setEditing(true)} sx={commonStyles}>
          {initialValue}
        </Typography>
      )}
    </Box>
  );
};
