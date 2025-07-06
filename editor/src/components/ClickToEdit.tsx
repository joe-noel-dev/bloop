import {Box, Input, Typography} from '@mui/joy';
import {useState, useEffect} from 'react';

interface Props {
  initialValue: string;
  size?: Size;
  endDecorator?: React.ReactNode;
  onSave: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  multiline?: boolean;
  validate?: (value: string) => string | null; // Returns error message or null
}

type Size = 'small' | 'medium' | 'large';

export const ClickToEdit = ({
  initialValue,
  size,
  endDecorator,
  onSave,
  placeholder,
  disabled = false,
  multiline = false,
  validate,
}: Props) => {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(initialValue);
  const [error, setError] = useState<string | null>(null);

  // Sync with prop changes
  useEffect(() => {
    setValue(initialValue);
    setError(null);
  }, [initialValue]);

  const submit = () => {
    const trimmedValue = value.trim();

    // Validate if validator provided
    if (validate) {
      const validationError = validate(trimmedValue);
      if (validationError) {
        setError(validationError);
        return;
      }
    }

    setError(null);
    if (trimmedValue !== initialValue) {
      onSave(trimmedValue);
    }
    setEditing(false);
  };

  const cancel = () => {
    setValue(initialValue);
    setError(null);
    setEditing(false);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setValue(newValue);

    // Clear error on change
    if (error) {
      setError(null);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !multiline) {
      e.preventDefault();
      submit();
    } else if (e.key === 'Enter' && multiline && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      submit();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancel();
    }
  };

  const commonStyles = {
    minHeight: size === 'large' ? 40 : size === 'medium' ? 32 : 28,
    display: 'flex',
    alignItems: 'center',
    width: '100%',
  };

  const editableStyles = {
    ...commonStyles,
    'cursor': disabled ? 'not-allowed' : 'pointer',
    'borderRadius': 'sm',
    'border': '1px solid transparent',
    'padding': '6px 8px',
    'transition': 'all 0.2s ease',
    '&:hover':
      !disabled && !editing
        ? {
            backgroundColor: 'background.level1',
            borderColor: 'neutral.300',
          }
        : {},
    '&:focus-within': {
      backgroundColor: 'background.surface',
      borderColor: 'primary.500',
      boxShadow: '0 0 0 2px var(--joy-palette-primary-100)',
    },
    'opacity': disabled ? 0.6 : 1,
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
    <Box sx={{width: '100%', position: 'relative'}}>
      {editing ? (
        <Input
          value={value}
          size={mapInputSize(size)}
          placeholder={
            placeholder || `Enter ${initialValue ? 'new value' : 'value'}...`
          }
          onChange={handleInputChange}
          onBlur={submit}
          onKeyDown={handleKeyDown}
          autoFocus
          color={error ? 'danger' : 'neutral'}
          sx={{
            ...commonStyles,
            '& input': {
              padding: '6px 8px',
            },
          }}
          endDecorator={endDecorator}
        />
      ) : (
        <Box
          onClick={() => !disabled && setEditing(true)}
          sx={editableStyles}
          role="button"
          tabIndex={disabled ? -1 : 0}
          onKeyDown={(e) => {
            if (!disabled && (e.key === 'Enter' || e.key === ' ')) {
              e.preventDefault();
              setEditing(true);
            }
          }}
          aria-label={`Click to edit ${initialValue || 'value'}`}
        >
          <Typography
            level={mapTypographySize(size)}
            endDecorator={endDecorator}
            sx={{
              color: !initialValue ? 'text.secondary' : 'inherit',
              fontStyle: !initialValue ? 'italic' : 'normal',
              flex: 1,
            }}
          >
            {initialValue || placeholder || 'Click to add...'}
          </Typography>
        </Box>
      )}
    </Box>
  );
};
