import {Divider, Stack} from '@mui/joy';
import {ProjectInfo} from './ProjectInfo';
import {Songs} from './Songs';
import {Transport} from '../transport/Transport';

export const Project = () => {
  return (
    <Stack padding={2} spacing={2}>
      <ProjectInfo />
      <Divider />
      <Transport />
      <Divider />
      <Songs />
    </Stack>
  );
};
