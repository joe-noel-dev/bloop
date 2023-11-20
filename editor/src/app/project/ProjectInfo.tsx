import {
  Button,
  List,
  ListItem,
  ListItemButton,
  ListItemContent,
  Modal,
  ModalClose,
  ModalDialog,
  Stack,
  Typography,
} from '@mui/joy';
import {useProject, useProjects} from '../../model-hooks/project-hooks';
import {Create, FolderOpen} from '@mui/icons-material';
import {useCore} from '../../core/use-core';
import {
  addProjectRequest,
  loadProjectRequest,
  loadProjectsRequest,
} from '../../api/request';
import {useState} from 'react';

export const ProjectInfo = () => {
  const project = useProject();
  const core = useCore();
  const [projectsModalOpen, setProjectsModalOpen] = useState(false);

  if (!core) {
    return <></>;
  }

  const create = () => {
    const request = addProjectRequest();
    core.sendRequest(request);
  };

  const openProjects = () => {
    const request = loadProjectsRequest();
    core.sendRequest(request);

    setProjectsModalOpen(true);
  };

  return (
    <Stack spacing={2}>
      <Typography level="title-lg" component="h1">
        {project?.info.name}
      </Typography>
      <Stack direction="row" spacing={2}>
        <Button startDecorator={<FolderOpen />} onClick={openProjects}>
          Projects
        </Button>
        <Button startDecorator={<Create />} onClick={create}>
          New Project
        </Button>
        <Modal
          open={projectsModalOpen}
          onClose={() => setProjectsModalOpen(false)}
        >
          <ModalDialog>
            <ModalClose />
            <ProjectsModal onRequestClose={() => setProjectsModalOpen(false)} />
          </ModalDialog>
        </Modal>
      </Stack>
    </Stack>
  );
};

interface ProjectsModalProps {
  onRequestClose: () => void;
}

const ProjectsModal = ({onRequestClose}: ProjectsModalProps) => {
  const projects = useProjects();
  const core = useCore();

  if (!core) {
    return <></>;
  }

  const loadProject = (projectId: string) => {
    const request = loadProjectRequest(projectId);
    core.sendRequest(request);
    onRequestClose();
  };

  return (
    <Stack spacing={1}>
      <Typography level="title-lg">Projects</Typography>

      <List sx={{overflow: 'scroll'}}>
        {projects.map((projectInfo) => (
          <ListItem key={projectInfo.id}>
            <ListItemButton
              variant="soft"
              onClick={() => loadProject(projectInfo.id)}
            >
              <ListItemContent>{projectInfo.name}</ListItemContent>
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </Stack>
  );
};
