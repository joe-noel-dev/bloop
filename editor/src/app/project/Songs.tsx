import {Divider, Stack, Typography} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';

export const Songs = () => {
  const songs = useSongs();

  return (
    <Stack spacing={2}>
      <Typography level="title-lg">Songs</Typography>
      <Stack spacing={2}>
        {songs &&
          songs.map((song) => (
            <Stack key={song.id}>
              <Song songId={song.id} />
              <Divider />
            </Stack>
          ))}
      </Stack>
    </Stack>
  );
};
