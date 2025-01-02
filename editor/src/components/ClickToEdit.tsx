import {Box, Input, Typography} from '@mui/joy';
import {useState} from 'react';

interface Props {
  initialValue: string;
  size?: Size;
  endDecorator?: React.ReactNode;
  onSave: (value: string) => void;
}

type Size = 'small' | 'medium' | 'large';

export const ClickToEdit = ({
  initialValue,
  size,
  endDecorator,
  onSave,
}: Props) => {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(initialValue);

  const submit = () => {
    onSave(value);
    setEditing(false);
  };

  const commonStyles = {
    padding: '4px',
  };

  const mapInputSize = (size?: Size) => {
    switch (size) {
      case 'small':
        return 'sm';
      case 'medium':
        return 'md';
      case 'large':
        return 'lg';
      default:
        return 'md';
    }
  };

  const mapTypographySize = (size?: Size) => {
    switch (size) {
      case 'small':
        return 'body-sm';
      case 'medium':
        return 'body-md';
      case 'large':
        return 'body-lg';
      default:
        return 'body-md';
    }
  };

  return (
    <Box sx={{width: '100%'}}>
      {editing ? (
        <Input
          value={value}
          size={mapInputSize(size)}
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
        <Typography
          onClick={() => setEditing(true)}
          sx={commonStyles}
          level={mapTypographySize(size)}
          endDecorator={endDecorator}
        >
          {initialValue}
        </Typography>
      )}
    </Box>
  );
};
