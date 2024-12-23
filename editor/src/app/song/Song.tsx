import {Button, Grid, IconButton, Input, Stack, Typography} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
import {
  Add,
  ArrowDownward,
  ArrowUpward,
  Cancel,
  Check,
  Delete,
  Edit,
} from '@mui/icons-material';
import {Sample} from '../sample/Sample';
import {Section} from '../section/Section';
import {
  addSectionRequest,
  removeSongRequest,
  updateSongRequest,
} from '../../api/request';
import {useCore} from '../../core/use-core';
import {useState} from 'react';
import {columnSize, columns} from '../section/TableInfo';
import {Song as ModelSong} from '../../model';

interface SongProps {
  songId: string;
  moveSong: (indexDelta: number) => void;
}

export const Song = ({songId, moveSong}: SongProps) => {
  const song = useSong(songId);
  const core = useCore();

  const [editing, setEditing] = useState(false);
  const [editingSongName, setEditingSongName] = useState<string>('');

  const [editingTempo, setEditingTempo] = useState<string>('');

  if (!song) {
    return <></>;
  }

  const addSection = () => {
    const request = addSectionRequest(songId);
    core.sendRequest(request);
  };

  const submit = () => {
    const newSong = {...song};

    if (editingSongName !== undefined) {
      newSong.name = editingSongName;
    }

    const tempo = parseFloat(editingTempo || '');
    if (!isNaN(tempo)) {
      newSong.tempo.bpm = tempo;
    }

    const request = updateSongRequest(newSong);
    core.sendRequest(request);

    setEditing(false);
  };

  const cancel = () => {
    setEditing(false);
  };

  const remove = () => {
    const request = removeSongRequest(songId);
    core.sendRequest(request);
  };

  const edit = () => {
    setEditing(true);
    setEditingSongName(song.name);
    setEditingTempo(`${song.tempo.bpm}`);
  };

  const moveUp = () => {
    moveSong(-1);
  };

  const moveDown = () => {
    moveSong(1);
  };

  return (
    <Stack spacing={2}>
      {editing ? (
        <form
          name="Song Details"
          onSubmit={(event) => {
            submit();
            event.preventDefault();
          }}
        >
          <Stack direction="row" spacing={2}>
            <SongNameInput
              value={editingSongName}
              onChange={setEditingSongName}
            />
            <SongTempoInput value={editingTempo} onChange={setEditingTempo} />
            <SubmitButton />
            <CancelButton onClick={cancel} />
            <RemoveButton onClick={remove} />
          </Stack>
        </form>
      ) : (
        <SongDetails
          song={song}
          onRequestEdit={edit}
          onMoveUp={moveUp}
          onMoveDown={moveDown}
        />
      )}
      <Sample sampleId={song.sample?.id || ''} songId={songId} />
      <SectionsTable song={song} onRequestAdd={addSection} />
    </Stack>
  );
};

interface SongDetailsProps {
  song: ModelSong;
  onRequestEdit: () => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
}

const SongDetails = ({
  song,
  onRequestEdit,
  onMoveUp,
  onMoveDown,
}: SongDetailsProps) => (
  <Stack direction="row" spacing={2} alignItems="center">
    <Typography level="title-lg">{song.name}</Typography>

    <Typography level="body-md">{song.tempo.bpm} bpm</Typography>

    <IconButton
      color="primary"
      size="sm"
      variant="soft"
      aria-label="Move song up"
      onClick={onMoveUp}
    >
      <ArrowUpward />
    </IconButton>

    <IconButton
      color="primary"
      size="sm"
      variant="soft"
      aria-label="Move song down"
      onClick={onMoveDown}
    >
      <ArrowDownward />
    </IconButton>

    <IconButton
      color="primary"
      size="sm"
      variant="soft"
      aria-label="Edit song name"
      onClick={onRequestEdit}
    >
      <Edit />
    </IconButton>
  </Stack>
);

interface InputProps<T> {
  value: T;
  onChange: (value: T) => void;
}
const SongNameInput = ({value, onChange}: InputProps<string>) => (
  <Input
    name="Song Name"
    sx={{width: 320}}
    value={value}
    onChange={(event) => onChange(event.target.value)}
  />
);

const SongTempoInput = ({value, onChange}: InputProps<string>) => (
  <Input
    sx={{width: 120}}
    name="Tempo"
    endDecorator={<Typography>bpm</Typography>}
    value={value}
    onChange={(event) => onChange(event.target.value)}
  />
);

const SubmitButton = () => (
  <IconButton name="Submit" color="success" variant="soft" type="submit">
    <Check />
  </IconButton>
);

const CancelButton = ({onClick}: {onClick: () => void}) => (
  <IconButton
    name="Cancel"
    color="warning"
    variant="soft"
    type="button"
    onClick={onClick}
  >
    <Cancel />
  </IconButton>
);

const RemoveButton = ({onClick}: {onClick: () => void}) => (
  <IconButton
    name="Remove"
    color="danger"
    variant="soft"
    type="button"
    onClick={onClick}
  >
    <Delete />
  </IconButton>
);

const TableHeader = () => (
  <Grid container spacing={1}>
    {columns.map((name) => (
      <Grid xs={columnSize(name)} key={name}>
        <Typography level="title-md">{name}</Typography>
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
  onRequestAdd,
}: {
  song: ModelSong;
  onRequestAdd: () => void;
}) => (
  <Stack spacing={1}>
    <TableHeader />

    {song.sections.map((section) => (
      <Section key={section.id} songId={song.id} sectionId={section.id} />
    ))}

    <TableFooter onRequestAdd={onRequestAdd} />
  </Stack>
);
