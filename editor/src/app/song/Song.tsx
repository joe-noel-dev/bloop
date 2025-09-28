import {useState} from 'react';
import {Button, Grid, Stack, Typography, Box} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
import {shadows, transitions} from '../../theme';
import {
  Add,
  ArrowDownward,
  ArrowUpward,
  Delete,
  DragIndicator,
} from '@mui/icons-material';
import {Sample} from '../sample/Sample';
import {Section} from '../section/Section';
import {columnSize, columns} from '../section/TableInfo';
import {ClickToEdit} from '../../components/ClickToEdit';
import {AbletonUpload} from './AbletonUpload';
import {ID, INVALID_ID, updateSectionBeatLength} from '../../api/helpers';
import {Song as ModelSong} from '../../api/bloop';
import {
  addSectionAction,
  removeSongAction,
  updateSongAction,
  moveSectionAction,
  splitSectionAction,
} from '../../dispatcher/action';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  DragEndEvent,
  useDroppable,
} from '@dnd-kit/core';
import {
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import {useSortable} from '@dnd-kit/sortable';
import {CSS} from '@dnd-kit/utilities';

interface SongProps {
  songId: ID;
  moveSong: (indexDelta: number) => void;
}

export const Song = ({songId, moveSong}: SongProps) => {
  const song = useSong(songId);
  const dispatch = useDispatcher();

  if (!song) {
    return <></>;
  }

  const addSection = () => dispatch(addSectionAction(song.id));
  const remove = () => dispatch(removeSongAction(song.id));
  const moveUp = () => moveSong(-1);
  const moveDown = () => moveSong(1);

  const updateSectionDuration = (sectionId: ID, duration: number) => {
    const newSong = {...song};
    updateSectionBeatLength(newSong, sectionId, duration);
    dispatch(updateSongAction(newSong));
  };

  const editSongName = (newName: string) =>
    dispatch(updateSongAction({...song, name: newName}));

  const editTempo = (newTempo: number) =>
    dispatch(updateSongAction({...song, tempo: {bpm: newTempo}}));

  return (
    <Stack spacing={2}>
      <SongDetails
        song={song}
        onEditName={editSongName}
        onEditTempo={editTempo}
        onMoveUp={moveUp}
        onMoveDown={moveDown}
        onRemove={remove}
      />
      <SectionsTable
        song={song}
        requestAdd={addSection}
        requestUpdateSectionDuration={updateSectionDuration}
      />
    </Stack>
  );
};

interface SongDetailsProps {
  song: ModelSong;
  onEditName: (newName: string) => void;
  onEditTempo: (newTempo: number) => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onRemove: () => void;
}

const SongDetails = ({
  song,
  onEditName,
  onEditTempo,
  onMoveUp,
  onMoveDown,
  onRemove,
}: SongDetailsProps) => (
  <Stack direction="column" spacing={2}>
    <ClickToEdit initialValue={song.name} onSave={onEditName} size="large" />
    <Stack direction={'row'} spacing={1} alignItems={'center'}>
      <ClickToEdit
        initialValue={`${song.tempo?.bpm ?? 120.0}`}
        onSave={(value) => onEditTempo(parseFloat(value))}
        size="medium"
        endDecorator={
          <Typography level="body-md" color="neutral">
            bpm
          </Typography>
        }
      />
    </Stack>

    <Stack direction="row" spacing={1}>
      <Button
        color="primary"
        size="sm"
        variant="soft"
        aria-label="Move song up"
        onClick={onMoveUp}
        startDecorator={<ArrowUpward />}
      >
        Move Up
      </Button>

      <Button
        color="primary"
        size="sm"
        variant="soft"
        aria-label="Move song down"
        onClick={onMoveDown}
        startDecorator={<ArrowDownward />}
      >
        Move Down
      </Button>

      <AbletonUpload songId={song.id} />

      <RemoveButton onClick={onRemove} />

      <Sample sampleId={song.sample?.id ?? INVALID_ID} songId={song.id} />
    </Stack>
  </Stack>
);

const RemoveButton = ({onClick}: {onClick: () => void}) => (
  <Button
    name="Remove"
    color="danger"
    variant="soft"
    type="button"
    onClick={onClick}
    startDecorator={<Delete />}
  >
    Remove Song
  </Button>
);

const TableHeader = () => (
  <Box
    sx={{
      backgroundColor: 'background.level1',
      borderRadius: 'sm',
      padding: 1.5,
      marginBottom: 1,
      border: '1px solid',
      borderColor: 'neutral.200',
    }}
  >
    <Grid container spacing={1}>
      {columns.map((name) => (
        <Grid xs={columnSize(name)} key={name}>
          <Typography
            level="title-sm"
            fontWeight={'bold'}
            sx={{
              color: 'text.secondary',
              textTransform: 'uppercase',
              letterSpacing: '0.5px',
              fontSize: '0.75rem',
            }}
          >
            {name}
          </Typography>
        </Grid>
      ))}
    </Grid>
  </Box>
);

const TableFooter = ({onRequestAdd}: {onRequestAdd: () => void}) => (
  <Box
    sx={{
      paddingTop: 2,
      borderTop: '1px solid',
      borderColor: 'neutral.200',
      marginTop: 1,
    }}
  >
    <Button
      startDecorator={<Add />}
      onClick={onRequestAdd}
      variant="soft"
      color="primary"
      size="sm"
      sx={{
        '&:hover': {
          transform: 'translateY(-1px)',
          boxShadow: shadows.hover,
        },
        'transition': transitions.fast,
      }}
    >
      Add Section
    </Button>
  </Box>
);

const DropZone = ({index}: {index: number}) => {
  const {isOver, setNodeRef} = useDroppable({
    id: `dropzone-${index}`,
  });

  return (
    <Box
      ref={setNodeRef}
      sx={{
        height: isOver ? 16 : 4,
        backgroundColor: isOver ? 'primary.500' : 'transparent',
        borderRadius: 'md',
        transition: transitions.easeOut,
        marginY: isOver ? 1 : 0.5,
        opacity: isOver ? 1 : 0.2,
        position: 'relative',
        border: isOver ? '2px dashed' : '1px dashed',
        borderColor: isOver ? 'primary.500' : 'neutral.300',
        ...(isOver && {
          '&::before': {
            content: '"Drop section here"',
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            fontSize: '0.75rem',
            color: 'white',
            fontWeight: 'bold',
            textShadow: shadows.level1,
            pointerEvents: 'none',
          },
          '&::after': {
            content: '""',
            position: 'absolute',
            top: -8,
            left: -8,
            right: -8,
            bottom: -8,
            backgroundColor: 'primary.100',
            borderRadius: 'lg',
            zIndex: -1,
            opacity: 0.3,
          },
        }),
      }}
    />
  );
};

const SortableSection = ({
  sectionId,
  songId,
  requestUpdateDuration,
}: {
  sectionId: ID;
  songId: ID;
  requestUpdateDuration: (sectionId: ID, duration: number) => void;
}) => {
  const {attributes, listeners, setNodeRef, transform, transition, isDragging} =
    useSortable({id: sectionId.toString()});

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.8 : 1,
    zIndex: isDragging ? 1000 : 'auto',
  };

  return (
    <Box
      ref={setNodeRef}
      style={style}
      sx={{
        'position': 'relative',
        'borderRadius': 'md',
        'transition': transitions.fast,
        'backgroundColor': 'background.surface',
        'border': '1px solid',
        'borderColor': 'neutral.200',
        'padding': 1.5,
        'marginY': 0.5,
        '&:hover': {
          'backgroundColor': 'background.level1',
          'borderColor': 'primary.300',
          'boxShadow': shadows.focus,
          '& .drag-handle': {
            opacity: 1,
          },
        },
        ...(isDragging && {
          backgroundColor: 'background.level2',
          borderColor: 'primary.500',
          boxShadow: 'lg',
          transform: 'rotate(2deg)',
        }),
      }}
    >
      <Box
        className="drag-handle"
        {...attributes}
        {...listeners}
        sx={{
          'position': 'absolute',
          'left': -40,
          'top': '50%',
          'transform': 'translateY(-50%)',
          'zIndex': 10,
          'cursor': 'grab',
          'opacity': 0,
          'transition': transitions.fast,
          'backgroundColor': 'background.surface',
          'borderRadius': 'md',
          'padding': 1,
          'border': '2px solid',
          'borderColor': 'neutral.300',
          'display': 'flex',
          'alignItems': 'center',
          'justifyContent': 'center',
          'width': 32,
          'height': 32,
          'boxShadow': shadows.active,
          '&:active': {
            cursor: 'grabbing',
            transform: 'translateY(-50%) scale(1.1)',
            boxShadow: shadows.elevated,
          },
          '&:hover': {
            borderColor: 'primary.500',
            backgroundColor: 'primary.50',
          },
        }}
      >
        <DragIndicator fontSize="small" sx={{color: 'neutral.600'}} />
      </Box>
      <Section
        songId={songId}
        sectionId={sectionId}
        requestUpdateDuration={requestUpdateDuration}
      />
    </Box>
  );
};

const SectionSplitter = ({
  previousSectionId,
  songId,
}: {
  previousSectionId: ID;
  songId: ID;
}) => {
  const dispatch = useDispatcher();
  const [isHovered, setIsHovered] = useState(false);

  const handleSplit = () => {
    dispatch(splitSectionAction(songId, previousSectionId));
  };

  return (
    <Box
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      sx={{
        position: 'relative',
        height: isHovered ? 40 : 8,
        transition: transitions.fast,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        cursor: 'pointer',
      }}
    >
      {isHovered && (
        <Button
          onClick={handleSplit}
          size="sm"
          variant="soft"
          color="primary"
          startDecorator={<Add />}
          sx={{
            'position': 'absolute',
            'fontSize': '0.75rem',
            'borderRadius': 'lg',
            'boxShadow': shadows.soft,
            '&:hover': {
              transform: 'scale(1.05)',
            },
            'transition': 'transform 0.1s ease',
          }}
        >
          Split Section
        </Button>
      )}
      <Box
        sx={{
          position: 'absolute',
          top: '50%',
          left: 0,
          right: 0,
          height: 2,
          backgroundColor: isHovered ? 'primary.300' : 'neutral.200',
          borderRadius: 'sm',
          transform: 'translateY(-50%)',
          transition: transitions.fast,
          zIndex: -1,
        }}
      />
    </Box>
  );
};

const SectionsTable = ({
  song,
  requestAdd,
  requestUpdateSectionDuration,
}: {
  song: ModelSong;
  requestAdd: () => void;
  requestUpdateSectionDuration: (sectionId: ID, duration: number) => void;
}) => {
  const dispatch = useDispatcher();

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragEnd = ({active, over}: DragEndEvent) => {
    if (!active.id || !over?.id) {
      return;
    }

    // Handle drop zones
    if (over.id.toString().startsWith('dropzone-')) {
      const dropIndex = parseInt(over.id.toString().replace('dropzone-', ''));
      const activeIndex = song.sections.findIndex(
        (section) => section.id.toString() === active.id
      );

      if (activeIndex !== -1) {
        const newIndex = dropIndex > activeIndex ? dropIndex - 1 : dropIndex;
        if (activeIndex !== newIndex) {
          dispatch(moveSectionAction(song.id, activeIndex, newIndex));
        }
      }
      return;
    }

    // Handle section-to-section drops
    const oldIndex = song.sections.findIndex(
      (section) => section.id.toString() === active.id
    );
    const newIndex = song.sections.findIndex(
      (section) => section.id.toString() === over.id
    );

    if (oldIndex !== newIndex) {
      dispatch(moveSectionAction(song.id, oldIndex, newIndex));
    }
  };

  return (
    <DndContext
      sensors={sensors}
      onDragEnd={handleDragEnd}
      collisionDetection={closestCenter}
    >
      <SortableContext
        items={song.sections.map((section) => section.id.toString())}
        strategy={verticalListSortingStrategy}
      >
        <Box
          sx={{
            paddingLeft: 6,
            position: 'relative',
            backgroundColor: 'background.body',
            borderRadius: 'lg',
            padding: 3,
            border: '1px solid',
            borderColor: 'neutral.200',
          }}
        >
          <Stack spacing={0}>
            <TableHeader />

            <Box sx={{minHeight: song.sections.length === 0 ? 120 : 'auto'}}>
              <DropZone index={0} />

              {song.sections.map((section, index) => (
                <Box key={section.id.toString()}>
                  <SortableSection
                    songId={song.id}
                    sectionId={section.id}
                    requestUpdateDuration={requestUpdateSectionDuration}
                  />
                  <DropZone index={index + 1} />
                  {index !== song.sections.length - 1 && (
                    <SectionSplitter
                      previousSectionId={section.id}
                      songId={song.id}
                    />
                  )}
                </Box>
              ))}

              {song.sections.length === 0 && (
                <Box
                  sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    justifyContent: 'center',
                    minHeight: 80,
                    color: 'text.secondary',
                    fontStyle: 'italic',
                  }}
                >
                  <Typography level="body-md">No sections yet</Typography>
                  <Typography level="body-sm">
                    Add your first section below
                  </Typography>
                </Box>
              )}
            </Box>

            <TableFooter onRequestAdd={requestAdd} />
          </Stack>
        </Box>
      </SortableContext>
    </DndContext>
  );
};
