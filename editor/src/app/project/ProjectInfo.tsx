import {
  Button,
  IconButton,
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
import {useProjectInfo, useProjects} from '../../model-hooks/project-hooks';
import {Create, Delete, FolderOpen} from '@mui/icons-material';
import {useState} from 'react';
import {useBackend} from '../../backend/Backend';

export const ProjectInfo = () => {
  const projectInfo = useProjectInfo();
  const backend = useBackend();
  const [projectsModalOpen, setProjectsModalOpen] = useState(false);

  const create = () => {
    // FIXME: create project
  };

  const openProjects = () => {
    setProjectsModalOpen(true);
  };

  return (
    <Stack spacing={2}>
      <Typography level="title-lg" component="h1">
        {projectInfo?.name || 'Untitled'}
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
  const backend = useBackend();

  const loadProject = async (projectId: string) => {
    await backend.loadProject(projectId);
    onRequestClose();
  };

  const removeProject = (projectId: string) => {
    // FIXME: remove project
    // backend.removeProject(projectId);
  };

  return (
    <Stack spacing={1}>
      <Typography level="title-lg">Projects</Typography>

      <List sx={{overflow: 'scroll'}}>
        {projects.map((projectInfo) => (
          <ListItem
            key={projectInfo.id.toString()}
            endAction={
              <IconButton
                aria-label="Delete"
                size="sm"
                color="danger"
                onClick={() => removeProject(projectInfo.id)}
              >
                <Delete />
              </IconButton>
            }
          >
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
