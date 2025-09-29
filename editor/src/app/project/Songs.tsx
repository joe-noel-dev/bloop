import {Button, Stack, Tab, TabList, TabPanel, Tabs} from '@mui/joy';
import {useSongs} from '../../model-hooks/song-hooks';
import {Song} from '../song/Song';
import {Add} from '@mui/icons-material';
import {useProject} from '../../model-hooks/project-hooks';
import Long from 'long';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  addSongAction,
  moveSongAction,
  selectSongAction,
} from '../../dispatcher/action';

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

  const moveSong = (fromIndex: number, toIndex: number) =>
    dispatcher(moveSongAction(fromIndex, toIndex));

  const onTabSelected = (
    _: React.SyntheticEvent | null,
    value: number | string | null
  ) => {
    if (typeof value !== 'string' || value === selectedSongId) {
      return;
    }
    const id = Long.fromString(value, true);
    dispatcher(selectSongAction(id));
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
        sx={{
          backgroundColor: 'background.surface',
          color: 'text.primary',
          borderRadius: 'md',
          padding: 1,
        }}
      >
        <TabList
          sx={{
            backgroundColor: 'background.level1',
            color: 'text.primary',
            borderRadius: 'sm',
          }}
        >
          {songs.map((song) => (
            <Tab
              key={song.id.toString()}
              value={song.id.toString()}
              sx={{
                'color': 'text.secondary',
                '&.Mui-selected': {
                  color: 'text.primary',
                  backgroundColor: 'background.surface',
                  fontWeight: 'bold',
                },
                '&:hover': {
                  color: 'text.primary',
                },
              }}
            >
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
