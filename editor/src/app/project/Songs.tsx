import {Divider, Stack} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';

export const Songs = () => {
  const songs = useSongs();

  return (
    <Stack spacing={2}>
      {songs &&
        songs.map((song) => (
          <Stack key={song.id} spacing={2}>
            <Song songId={song.id} />
            <Divider />
          </Stack>
        ))}
    </Stack>
  );
};
