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

  return (
    <Stack spacing={2}>
      {!editing ? (
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
      ) : (
        <form
          onSubmit={(event) => {
            submit();
            event.preventDefault();
          }}
        >
          <Stack direction="row" spacing={2}>
            <Input
              sx={{width: 320}}
              value={editingSongName}
              onChange={(event) => setEditingSongName(event.target.value)}
            />

            <Input
              sx={{width: 120}}
              endDecorator={<Typography>bpm</Typography>}
              value={editingTempo}
              onChange={(event) => setEditingTempo(event.target.value)}
            />

            <IconButton color="success" variant="soft" type="submit">
              <Check />
            </IconButton>

            <IconButton
              color="warning"
              variant="soft"
              type="button"
              onClick={cancel}
            >
              <Cancel />
            </IconButton>
          </Stack>
        </form>
      )}

      <Sample sampleId={song.sample?.id || ''} songId={songId} />

      <Table
        sx={{
          '--TableCell-selectedBackground': (theme) =>
            theme.vars.palette.primary.softBg,
        }}
      >
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
        <tbody>
          {song.sections.map((section) => (
            <Section key={section.id} sectionId={section.id} />
          ))}
        </tbody>
        <tfoot>
          <tr>
            <td colSpan={6}>
              <Button startDecorator={<Add />} onClick={addSection}>
                Add Section
              </Button>
            </td>
          </tr>
        </tfoot>
      </Table>
    </Stack>
  );
};
