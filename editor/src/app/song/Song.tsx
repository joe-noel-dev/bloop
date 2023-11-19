import {Button, IconButton, Input, Stack, Table, Typography} from '@mui/joy';
import {useSong} from '../../model-hooks/song-hooks';
import {Add, Cancel, Check, Edit} from '@mui/icons-material';
import {Sample} from '../sample/Sample';
import {Section} from '../section/Section';
import {addSectionRequest, updateSongRequest} from '../../api/request';
import {useCore} from '../../core/use-core';
import {useState} from 'react';

interface SongProps {
  songId: string;
}

export const Song = ({songId}: SongProps) => {
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

  const edit = () => {
    setEditing(true);
    setEditingSongName(song.name);
    setEditingTempo(`${song.tempo.bpm}`);
  };

  const SongDetails = () => (
    <Stack direction="row" spacing={2} alignItems="center">
      <Typography level="title-lg">{song.name}</Typography>

      <Typography level="body-md">{song.tempo.bpm} bpm</Typography>

      <IconButton
        color="primary"
        size="sm"
        variant="soft"
        aria-label="Edit song name"
        onClick={edit}
      >
        <Edit />
      </IconButton>
    </Stack>
  );

  const SongNameInput = () => (
    <Input
      name="Song Name"
      sx={{width: 320}}
      value={editingSongName}
      onChange={(event) => setEditingSongName(event.target.value)}
    />
  );

  const SongTempoInput = () => (
    <Input
      sx={{width: 120}}
      name="Tempo"
      endDecorator={<Typography>bpm</Typography>}
      value={editingTempo}
      onChange={(event) => setEditingTempo(event.target.value)}
    />
  );

  const SubmitButton = () => (
    <IconButton name="Submit" color="success" variant="soft" type="submit">
      <Check />
    </IconButton>
  );

  const CancelButton = () => (
    <IconButton
      name="Cancel"
      color="warning"
      variant="soft"
      type="button"
      onClick={cancel}
    >
      <Cancel />
    </IconButton>
  );

  const EditSongDetails = () => (
    <form
      name="Song Details"
      onSubmit={(event) => {
        submit();
        event.preventDefault();
      }}
    >
      <Stack direction="row" spacing={2}>
        <SongNameInput />
        <SongTempoInput />
        <SubmitButton />
        <CancelButton />
      </Stack>
    </form>
  );

  const TableHeader = () => (
    <thead>
      <tr>
        <th style={{width: '48px'}}></th>
        <th>Name</th>
        <th>Start (beats)</th>
        <th>Loop</th>
        <th>Metronome</th>
        <th>Edit</th>
      </tr>
    </thead>
  );

  const TableBody = () => (
    <tbody>
      {song.sections.map((section) => (
        <Section key={section.id} sectionId={section.id} />
      ))}
    </tbody>
  );

  const TableFooter = () => (
    <tfoot>
      <tr>
        <td colSpan={6}>
          <Button startDecorator={<Add />} onClick={addSection}>
            Add Section
          </Button>
        </td>
      </tr>
    </tfoot>
  );

  const SectionsTable = () => (
    <Table
      sx={{
        '--TableCell-selectedBackground': (theme) =>
          theme.vars.palette.primary.softBg,
      }}
    >
      <TableHeader />
      <TableBody />
      <TableFooter />
    </Table>
  );

  return (
    <Stack spacing={2}>
      {editing ? <EditSongDetails /> : <SongDetails />}
      <Sample sampleId={song.sample?.id || ''} songId={songId} />
      <SectionsTable />
    </Stack>
  );
};
