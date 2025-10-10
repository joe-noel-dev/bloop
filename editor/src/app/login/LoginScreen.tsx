import {useState} from 'react';
import {
  FormControl,
  FormLabel,
  Input,
  Button,
  Typography,
  Box,
  Alert,
} from '@mui/joy';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {signInAction} from '../../dispatcher/action';

export const LoginScreen = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const dispatch = useDispatcher();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!username || !password) {
      setError('Please enter both username and password.');
      return;
    }
    setError('');

    dispatch(signInAction(username, password));
  };

  return (
    <Box
      sx={{
        maxWidth: 340,
        mx: 'auto',
        my: 6,
        p: 3,
        borderRadius: 8,
        boxShadow: 'md',
        bgcolor: 'background.body',
      }}
    >
      <Typography level="h4" textAlign="center" mb={2}>
        Login
      </Typography>
      <form onSubmit={handleSubmit} id="login-form" name="login-form">
        <FormControl sx={{mb: 2}}>
          <FormLabel>Username</FormLabel>
          <Input
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            autoComplete="username"
            required
          />
        </FormControl>
        <FormControl sx={{mb: 2}}>
          <FormLabel>Password</FormLabel>
          <Input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            autoComplete="current-password"
            required
          />
        </FormControl>
        {error && (
          <Alert color="danger" variant="soft" sx={{mb: 2}}>
            {error}
          </Alert>
        )}
        <Button type="submit" fullWidth variant="solid" color="primary">
          Log In
        </Button>
      </form>
    </Box>
  );
};
