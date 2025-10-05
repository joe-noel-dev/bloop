import {
  Button,
  CircularProgress,
  IconButton,
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
import {transitions, spacing, backdrop, opacity} from '../../theme';
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
    <Stack spacing={spacing.stackSpacing2}>
      <ClickToEdit
        size="large"
        initialValue={projectInfo?.name || ''}
        onSave={renameProject}
      />
      <Stack direction="row" spacing={spacing.stackSpacing2}>
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
          slotProps={{
            backdrop: {
              sx: {
                backgroundColor: backdrop.default,
                backdropFilter: backdrop.blur,
              },
            },
          }}
        >
          <ModalDialog
            aria-labelledby="projects-modal-title"
            sx={{
              backgroundColor: 'background.surface',
              color: 'text.primary',
              border: '1px solid',
              borderColor: 'neutral.300',
              minWidth: spacing.modalMinWidth,
              maxWidth: spacing.modalMaxWidth,
              maxHeight: spacing.modalMaxHeight,
            }}
          >
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

  // Sort projects by creation date (newest first)
  const sortedProjects = [...projects].sort(
    (a, b) => new Date(b.created).getTime() - new Date(a.created).getTime()
  );

  return (
    <Stack
      spacing={spacing.stackSpacing3}
      sx={{color: 'text.primary', width: '100%'}}
    >
      <Typography
        level="h4"
        id="projects-modal-title"
        sx={{color: 'text.primary', textAlign: 'center'}}
      >
        Projects
      </Typography>

      {sortedProjects.length === 0 ? (
        // Empty state
        <Stack
          spacing={spacing.stackSpacing2}
          sx={{
            alignItems: 'center',
            py: spacing.emptyStatePaddingY,
            px: spacing.emptyStatePaddingX,
            textAlign: 'center',
            color: 'text.secondary',
          }}
        >
          <FolderOpen
            sx={{
              fontSize: spacing.emptyStateIconSize,
              color: 'neutral.400',
              opacity: opacity.hover,
            }}
          />
          <Typography level="body-lg" sx={{color: 'text.secondary'}}>
            No projects yet
          </Typography>
          <Typography level="body-sm" sx={{color: 'text.tertiary'}}>
            Click "New Project" to create your first project
          </Typography>
        </Stack>
      ) : (
        // Projects list
        <Stack
          role="list"
          aria-label="Projects list"
          sx={{
            maxHeight: spacing.modalListMaxHeight,
            overflow: 'auto',
            backgroundColor: 'background.level1',
            borderRadius: 'md',
            border: '1px solid',
            borderColor: 'neutral.300',
          }}
        >
          {sortedProjects.map((projectInfo, index) => (
            <Stack
              key={projectInfo.id}
              direction="row"
              role="listitem"
              sx={{
                'alignItems': 'center',
                'p': spacing.layoutPadding,
                'backgroundColor': 'background.surface',
                'borderBottom':
                  index < sortedProjects.length - 1 ? '1px solid' : 'none',
                'borderBottomColor': 'neutral.200',
                'transition': transitions.fast,
                '&:hover': {
                  backgroundColor: 'background.level2',
                },
                '&:first-of-type': {
                  borderTopLeftRadius: 'md',
                  borderTopRightRadius: 'md',
                },
                '&:last-of-type': {
                  borderBottomLeftRadius: 'md',
                  borderBottomRightRadius: 'md',
                },
              }}
            >
              <Stack
                onClick={() => loadProject(projectInfo.id)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    loadProject(projectInfo.id);
                  }
                }}
                tabIndex={0}
                role="button"
                aria-label={`Open project ${projectInfo.name}`}
                sx={{
                  'flexGrow': 1,
                  'cursor': 'pointer',
                  'minWidth': 0,
                  'outline': 'none',
                  'borderRadius': 'sm',
                  '&:focus-visible': {
                    outline: `${spacing.focusOutlineWidth}px solid`,
                    outlineColor: 'primary.500',
                    outlineOffset: `${spacing.focusOutlineOffset}px`,
                  },
                }}
              >
                <Typography
                  level="title-md"
                  sx={{
                    color: 'text.primary',
                    fontWeight: 'md',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis',
                    whiteSpace: 'nowrap',
                  }}
                >
                  {projectInfo.name}
                </Typography>
                <Typography
                  level="body-sm"
                  sx={{
                    color: 'text.tertiary',
                    mt: spacing.textMarginTop,
                  }}
                >
                  Created {new Date(projectInfo.created).toLocaleDateString()}
                </Typography>
              </Stack>

              <IconButton
                aria-label="Delete project"
                size="sm"
                color="danger"
                variant="plain"
                onClick={(e) => {
                  e.stopPropagation();
                  removeProject(projectInfo.id);
                }}
                sx={{
                  'ml': spacing.modalMarginLeft,
                  'opacity': opacity.hover,
                  'transition': transitions.fast,
                  '&:hover': {
                    opacity: opacity.active,
                    backgroundColor: 'danger.100',
                  },
                }}
              >
                <Delete />
              </IconButton>
            </Stack>
          ))}
        </Stack>
      )}
    </Stack>
  );
};
