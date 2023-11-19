import {Divider, Stack} from '@mui/joy';
import {ProjectInfo} from './ProjectInfo';
import {Songs} from './Songs';
import {useState} from 'react';
import {EditingSectionContext} from './EditingSectionContext';

export const Project = () => {
  const [editingSectionId, setEditingSectionId] = useState('');

  return (
    <Stack padding={2} spacing={2}>
      <ProjectInfo />
      <Divider />
      <EditingSectionContext.Provider
        value={[editingSectionId, setEditingSectionId]}
      >
        <Songs />
      </EditingSectionContext.Provider>
    </Stack>
  );
};
