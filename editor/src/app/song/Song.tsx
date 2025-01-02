import {Button, Grid, Stack, Typography} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
import {Add, ArrowDownward, ArrowUpward, Delete} from '@mui/icons-material';
import {Sample} from '../sample/Sample';
import {Section} from '../section/Section';
import {
  addSectionRequest,
  removeSongRequest,
  updateSongRequest,
} from '../../api/request';
import {useCore} from '../../core/use-core';
import {columnSize, columns} from '../section/TableInfo';
import {Song as ModelSong} from '../../model';
import {updateSectionBeatLength} from '../../model/song';
import {ClickToEdit} from '../../components/ClickToEdit';

interface SongProps {
  songId: string;
  moveSong: (indexDelta: number) => void;
}

export const Song = ({songId, moveSong}: SongProps) => {
  const song = useSong(songId);
  const core = useCore();

  if (!song) {
    return <></>;
  }

  const addSection = () => {
    const request = addSectionRequest(songId);
    core.sendRequest(request);
  };

  const remove = () => {
    const request = removeSongRequest(songId);
    core.sendRequest(request);
  };

  const moveUp = () => {
    moveSong(-1);
  };

  const moveDown = () => {
    moveSong(1);
  };

  const updateSectionDuration = (sectionId: string, duration: number) => {
    const newSong = {...song};
    updateSectionBeatLength(newSong, sectionId, duration);
    const request = updateSongRequest(newSong);
    core.sendRequest(request);
  };

  const editSongName = (newName: string) => {
    const newSong = {...song, name: newName};
    const request = updateSongRequest(newSong);
    core.sendRequest(request);
  };

  const editTempo = (newTempo: number) => {
    const newSong = {...song, tempo: {bpm: newTempo}};
    const request = updateSongRequest(newSong);
    core.sendRequest(request);
  };

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
        initialValue={`${song.tempo.bpm}`}
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

      <RemoveButton onClick={onRemove} />

      <Sample sampleId={song.sample?.id || ''} songId={song.id} />
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
  requestUpdateSectionDuration: (sectionId: string, duration: number) => void;
}) => (
  <Stack spacing={1}>
    <TableHeader />

    {song.sections.map((section) => (
      <Section
        key={section.id}
        songId={song.id}
        sectionId={section.id}
        requestUpdateDuration={requestUpdateSectionDuration}
      />
    ))}

    <TableFooter onRequestAdd={requestAdd} />
  </Stack>
);
