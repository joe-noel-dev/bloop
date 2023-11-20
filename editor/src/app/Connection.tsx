import {Check} from '@mui/icons-material';
import {Button, Chip, Input, Stack} from '@mui/joy';
import {useState} from 'react';

interface ConnectionProps {
  isConnected: boolean;
  connect: (address: string) => void;
  disconnect: () => void;
}

export const Connection = (props: ConnectionProps) => {
  const [wsAddress, setWsAddress] = useState('');

  const submit = () => {
    if (props.isConnected) {
      props.disconnect();
    } else {
      props.connect(
        wsAddress.startsWith('ws://') ? wsAddress : `ws://${wsAddress}`
      );
    }
  };

  return (
    <form
      name="Core Address"
      onSubmit={(event) => {
        submit();
        event.preventDefault();
      }}
    >
      <Stack direction="row" sx={{padding: 2}} spacing={2}>
        <Input
          name="WebSocket Address"
          placeholder="WebSocket Address"
          value={wsAddress}
          onChange={(event) => setWsAddress(event.target.value)}
        />

        <Button name="Connect" onClick={submit}>
          {props.isConnected ? 'Disconnect' : 'Connect'}
        </Button>

        {props.isConnected && (
          <Chip color="success" startDecorator={<Check />} sx={{paddingX: 2}}>
            Connected
          </Chip>
        )}
      </Stack>
    </form>
  );
};
