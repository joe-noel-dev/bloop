import {Button, Divider, Stack} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';
import {Add} from '@mui/icons-material';
import {useCore} from '../../core/use-core';
import {addSongRequest} from '../../api/request';

export const Songs = () => {
  const songs = useSongs();
  const core = useCore();

  if (!core) {
    return <></>;
  }

  const addSong = () => {
    const request = addSongRequest();
    core.sendRequest(request);
  };

  return (
    <Stack spacing={2}>
      {songs &&
        songs.map((song) => (
          <Stack key={song.id} spacing={2}>
            <Song songId={song.id} />
            <Divider />
          </Stack>
        ))}

      <Stack direction="row">
        <Button startDecorator={<Add />} onClick={addSong}>
          Add Song
        </Button>
      </Stack>
    </Stack>
  );
};
