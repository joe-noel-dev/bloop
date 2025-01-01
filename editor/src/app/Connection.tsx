import {Check} from '@mui/icons-material';
import {Button, Chip, Option, Select, Stack} from '@mui/joy';
import {useEffect, useState} from 'react';

interface ConnectionProps {
  isConnected: boolean;
  connect: (address: string) => void;
  disconnect: () => void;
}

interface Service {
  name: string;
  addresses: string[];
  port: number;
  host: string;
  networkInterface: string;
}

export const Connection = (props: ConnectionProps) => {
  const [wsAddress, setWsAddress] = useState('');
  const [services, setServices] = useState<Service[]>([]);

  const connect = (address: string) => {
    props.connect(address.startsWith('ws://') ? address : `ws://${address}`);
  };

  const scan = async () => {
    try {
      const response = await fetch('/api/discover');
      const responseJSON = await response.json();

      let foundServices = responseJSON.services as Service[];

      foundServices.forEach((service) => {
        service.addresses = service.addresses.filter((address) =>
          address.includes('.')
        );
      });

      foundServices = foundServices.filter(
        (service) => service.addresses.length
      );

      setServices(foundServices);
      console.log('Found services: ', foundServices);

      if (foundServices.length === 1 && !props.isConnected) {
        const service = foundServices[0];
        const address = `${service.host}:${service.port}`;
        setWsAddress(address);
        connect(address);
      }
    } catch (error) {
      console.error('Failed to scan for services', error);
    }
  };

  useEffect(() => {
    scan();
  }, []);

  const onServiceSelect = (
    _: React.SyntheticEvent | null,
    newValue: string | null
  ) => {
    console.log('Selected service: ', newValue);
    setWsAddress(newValue ?? '');
  };

  return (
    <Stack direction="row" sx={{padding: 2}} spacing={2}>
      {!props.isConnected && (
        <Button name="Scan" onClick={scan}>
          Scan
        </Button>
      )}

      {!props.isConnected && services.length > 1 && (
        <Select placeholder="Core" onChange={onServiceSelect}>
          {services.map((service) => (
            <Option
              key={`${service.networkInterface}/${service.addresses[0]}/${service.port}}`}
              value={`${service.host}:${service.port}`}
            >
              {service.name} ({service.networkInterface})
            </Option>
          ))}
        </Select>
      )}

      {props.isConnected && (
        <Button
          name="Disconnect"
          onClick={() => {
            props.disconnect();
            setWsAddress('');
          }}
        >
          Disconnect
        </Button>
      )}

      {!props.isConnected && wsAddress && (
        <Button name="Connect" onClick={() => connect(wsAddress)}>
          Connect
        </Button>
      )}

      {props.isConnected && (
        <Chip color="success" startDecorator={<Check />} sx={{paddingX: 2}}>
          Connected
        </Chip>
      )}
    </Stack>
  );
};
