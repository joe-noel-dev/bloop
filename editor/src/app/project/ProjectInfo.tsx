import {
  Button,
  CircularProgress,
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
import {
  useProjectInfo,
  useProjects,
  useSaveState,
} from '../../model-hooks/project-hooks';
import {
  Create,
  Delete,
  FolderOpen,
  Save,
  CheckCircle,
} from '@mui/icons-material';
import {useState} from 'react';
import {ClickToEdit} from '../../components/ClickToEdit';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  createProjectAction,
  loadProjectAction,
  removeProjectAction,
  renameProjectAction,
  loadProjectsAction,
  saveProjectAction,
} from '../../dispatcher/action';

export const ProjectInfo = () => {
  const projectInfo = useProjectInfo();
  const saveState = useSaveState();
  const [projectsModalOpen, setProjectsModalOpen] = useState(false);
  const dispatch = useDispatcher();

  const create = () => dispatch(createProjectAction());

  const openProjects = () => {
    dispatch(loadProjectsAction());
    setProjectsModalOpen(true);
  };

  const save = () => dispatch(saveProjectAction());

  const renameProject = (name: string) => dispatch(renameProjectAction(name));

  const getSaveButtonProps = () => {
    switch (saveState) {
      case 'saving':
        return {
          startDecorator: <CircularProgress size="sm" />,
          children: 'Saving...',
          disabled: true,
        };
      case 'saved':
        return {
          startDecorator: <CheckCircle sx={{color: 'success.main'}} />,
          children: 'Saved!',
          disabled: false,
        };
      default:
        return {
          startDecorator: <Save />,
          children: 'Save Project',
          disabled: false,
        };
    }
  };

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
        <Button onClick={save} {...getSaveButtonProps()} />
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

  const loadProject = (projectId: string) => {
    dispatch(loadProjectAction(projectId));
    onRequestClose();
  };

  const removeProject = (projectId: string) => {
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
