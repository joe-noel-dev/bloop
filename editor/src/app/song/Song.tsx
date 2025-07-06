import {Button, Grid, Stack, Typography, Box} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
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
  DragOverlay,
  DragStartEvent,
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
  <Grid container spacing={1}>
    {columns.map((name) => (
      <Grid xs={columnSize(name)} key={name}>
        <Typography level="title-md" fontWeight={'bold'}>
          {name}
        </Typography>
      </Grid>
    ))}
  </Grid>
);

const TableFooter = ({onRequestAdd}: {onRequestAdd: () => void}) => (
  <Stack direction="row">
    <Button startDecorator={<Add />} onClick={onRequestAdd}>
      Add Section
    </Button>
  </Stack>
);

const DropZone = ({index}: {index: number}) => {
  const {isOver, setNodeRef} = useDroppable({
    id: `dropzone-${index}`,
  });

  return (
    <Box
      ref={setNodeRef}
      sx={{
        height: isOver ? 8 : 1,
        backgroundColor: isOver ? 'primary.500' : 'neutral.200',
        borderRadius: 'sm',
        transition: 'all 0.2s ease',
        marginY: isOver ? 1 : 0.25,
        opacity: isOver ? 1 : 0.3,
        position: 'relative',
        ...(isOver && {
          '&::before': {
            content: '""',
            position: 'absolute',
            top: -2,
            left: -4,
            right: -4,
            bottom: -2,
            backgroundColor: 'primary.100',
            borderRadius: 'md',
            zIndex: -1,
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
        'borderRadius': 'sm',
        'transition': 'all 0.2s ease',
        '&:hover': {
          'backgroundColor': 'background.level1',
          '& .drag-handle': {
            opacity: 1,
          },
        },
        ...(isDragging && {
          backgroundColor: 'background.level2',
          boxShadow: 'lg',
        }),
      }}
    >
      <Box
        className="drag-handle"
        {...attributes}
        {...listeners}
        sx={{
          'position': 'absolute',
          'left': -35,
          'top': '50%',
          'transform': 'translateY(-50%)',
          'zIndex': 10,
          'cursor': 'grab',
          'opacity': 0,
          'transition': 'all 0.2s ease',
          'backgroundColor': 'background.surface',
          'borderRadius': 'sm',
          'padding': 0.5,
          'border': '1px solid',
          'borderColor': 'neutral.300',
          'display': 'flex',
          'alignItems': 'center',
          'justifyContent': 'center',
          'width': 28,
          'height': 28,
          '&:active': {
            cursor: 'grabbing',
            transform: 'translateY(-50%) scale(1.1)',
          },
          '&:hover': {
            borderColor: 'primary.500',
            backgroundColor: 'primary.50',
          },
        }}
      >
        <DragIndicator fontSize="small" sx={{color: 'neutral.600'}} />
      </Box>
      <Box sx={{marginY: 0.5}}>
        <Section
          songId={songId}
          sectionId={sectionId}
          requestUpdateDuration={requestUpdateDuration}
        />
      </Box>
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
        <Box sx={{paddingLeft: 5, position: 'relative'}}>
          <Stack spacing={0}>
            <TableHeader />

            <DropZone index={0} />

            {song.sections.map((section, index) => (
              <Box key={section.id.toString()}>
                <SortableSection
                  songId={song.id}
                  sectionId={section.id}
                  requestUpdateDuration={requestUpdateSectionDuration}
                />
                <DropZone index={index + 1} />
              </Box>
            ))}

            <TableFooter onRequestAdd={requestAdd} />
          </Stack>
        </Box>
      </SortableContext>
    </DndContext>
  );
};
