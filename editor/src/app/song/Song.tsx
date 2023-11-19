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

  const [editingSongName, setEditingSongName] = useState<string | undefined>(
    undefined
  );

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

    const request = updateSongRequest(newSong);
    core.sendRequest(request);

    setEditingSongName(undefined);
  };

  const cancel = () => setEditingSongName(undefined);

  return (
    <Stack spacing={2}>
      {editingSongName === undefined ? (
        <Typography
          level="title-md"
          endDecorator={
            <IconButton
              color="primary"
              size="sm"
              variant="soft"
              aria-label="Edit song name"
              onClick={() => setEditingSongName(song.name)}
            >
              <Edit />
            </IconButton>
          }
        >
          {song.name}
        </Typography>
      ) : (
        <Stack direction="row" spacing={2}>
          <Input
            value={editingSongName}
            onChange={(event) => setEditingSongName(event.target.value)}
          />
          <IconButton color="success" variant="soft" onClick={submit}>
            <Check />
          </IconButton>
          <IconButton color="warning" variant="soft" onClick={cancel}>
            <Cancel />
          </IconButton>
        </Stack>
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
            <td colSpan={5}>
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
