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
import {ClickToEdit} from '../../components/ClickToEdit';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  createProjectAction,
  loadProjectAction,
  removeProjectAction,
  renameProjectAction,
} from '../../dispatcher/action';

export const ProjectInfo = () => {
  const projectInfo = useProjectInfo();
  const [projectsModalOpen, setProjectsModalOpen] = useState(false);
  const dispatch = useDispatcher();

  const create = async () => dispatch(createProjectAction());
  const openProjects = () => setProjectsModalOpen(true);
  const renameProject = (name: string) => dispatch(renameProjectAction(name));

  return (
    <Stack spacing={2}>
      <ClickToEdit
        size="large"
        initialValue={projectInfo?.name || ''}
        onSave={renameProject}
      />
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
  const dispatch = useDispatcher();

  const loadProject = async (projectId: string) => {
    dispatch(loadProjectAction(projectId));
    onRequestClose();
  };

  const removeProject = async (projectId: string) => {
    if (!window.confirm('Are you sure you want to delete this project?')) {
      return;
    }

    dispatch(removeProjectAction(projectId));
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
