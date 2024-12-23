import {Button, Divider, Stack} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';
import {Add} from '@mui/icons-material';
import {useCore} from '../../core/use-core';
import {addSongRequest, updateProjectRequest} from '../../api/request';
import {useProject} from '../../model-hooks/project-hooks';

export const Songs = () => {
  const songs = useSongs() || [];
  const project = useProject();
  const core = useCore();

  if (!core || !project) {
    return <></>;
  }

  const addSong = () => {
    const request = addSongRequest();
    core.sendRequest(request);
  };

  const moveSong = (fromIndex: number, toIndex: number) => {
    if (toIndex < 0 || toIndex >= songs.length) {
      return;
    }

    const newSongs = [...songs];
    newSongs.splice(toIndex, 0, newSongs.splice(fromIndex, 1)[0]);

    const request = updateProjectRequest({
      ...project,
      songs: newSongs,
    });

    core.sendRequest(request);
  };

  return (
    <Stack spacing={2}>
      {songs &&
        songs.map((song, index) => (
          <Stack key={song.id} spacing={2}>
            <Song
              songId={song.id}
              moveSong={(delta) => moveSong(index, index + delta)}
            />
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
