import {Button, Stack, Tab, TabList, TabPanel, Tabs} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';
import {Add} from '@mui/icons-material';
import {useCore} from '../../core/use-core';
import {
  addSongRequest,
  selectSongRequest,
  updateProjectRequest,
} from '../../api/request';
import {useProject} from '../../model-hooks/project-hooks';
import Long from 'long';

export const Songs = () => {
  const songs = useSongs() || [];
  const project = useProject();
  const core = useCore();

  if (!core || !project) {
    return <></>;
  }

  const selectedSongId = project.selections?.song || '';

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

  const onTabSelected = (
    _: React.SyntheticEvent | null,
    value: number | string | null
  ) => {
    if (typeof value !== 'string' || value === selectedSongId) {
      return;
    }
    const id = Long.fromString(value);
    const request = selectSongRequest(id);
    core.sendRequest(request);
  };

  return (
    <Stack spacing={2}>
      <Stack direction="row" spacing={2}>
        <Button startDecorator={<Add />} onClick={addSong}>
          Add Song
        </Button>
      </Stack>

      <Tabs
        orientation="vertical"
        value={selectedSongId.toString()}
        onChange={onTabSelected}
      >
        <TabList>
          {songs.map((song) => (
            <Tab key={song.id.toString()} value={song.id.toString()}>
              {song.name}
            </Tab>
          ))}
        </TabList>
        {songs.map((song, index) => (
          <TabPanel key={song.id.toString()} value={song.id.toString()}>
            <Song
              songId={song.id}
              moveSong={(delta) => moveSong(index, index + delta)}
            />
          </TabPanel>
        ))}
      </Tabs>
    </Stack>
  );
};
