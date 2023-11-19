import {Box, Typography} from '@mui/joy';
import {useProject} from '../../model-hooks/project-hooks';

export const ProjectInfo = () => {
  const project = useProject();

  return (
    <Box>
      <Typography level="title-lg">{project?.info.name}</Typography>
    </Box>
  );
};
