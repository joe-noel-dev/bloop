import {Button, Stack, Tab, TabList, TabPanel, Tabs} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';
import {Add} from '@mui/icons-material';
import {useProject} from '../../model-hooks/project-hooks';
import Long from 'long';
import {useDispatcher} from '../../dispatcher/Dispatcher';
import {addSongAction} from '../../dispatcher/action';

export const Songs = () => {
  const songs = useSongs() || [];
  const project = useProject();
  const dispatcher = useDispatcher();

  if (!project) {
    return <></>;
  }

  const selectedSongId = project.selections?.song || '';

  const addSong = () => {
    dispatcher(addSongAction());
  };

  const moveSong = (fromIndex: number, toIndex: number) => {
    if (toIndex < 0 || toIndex >= songs.length) {
      return;
    }

    const newSongs = [...songs];
    newSongs.splice(toIndex, 0, newSongs.splice(fromIndex, 1)[0]);

    const newProject = {
      ...project,
      songs: newSongs,
    };

    // FIXME: update project
  };

  const onTabSelected = (
    _: React.SyntheticEvent | null,
    value: number | string | null
  ) => {
    if (typeof value !== 'string' || value === selectedSongId) {
      return;
    }
    const id = Long.fromString(value, true);
    // FIXME: select song
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
