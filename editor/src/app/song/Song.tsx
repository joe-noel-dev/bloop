import {Button, Grid, Stack, Typography} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
import {Add, ArrowDownward, ArrowUpward, Delete} from '@mui/icons-material';
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
} from '../../dispatcher/action';
import {useDispatcher} from '../../dispatcher/dispatcher';

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

const SectionsTable = ({
  song,
  requestAdd,
  requestUpdateSectionDuration,
}: {
  song: ModelSong;
  requestAdd: () => void;
  requestUpdateSectionDuration: (sectionId: ID, duration: number) => void;
}) => (
  <Stack spacing={1}>
    <TableHeader />

    {song.sections.map((section) => (
      <Section
        key={section.id.toString()}
        songId={song.id}
        sectionId={section.id}
        requestUpdateDuration={requestUpdateSectionDuration}
      />
    ))}

    <TableFooter onRequestAdd={requestAdd} />
  </Stack>
);
