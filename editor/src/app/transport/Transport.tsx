import {
  Box,
  ButtonGroup,
  Card,
  IconButton,
  LinearProgress,
  Stack,
  Typography,
} from '@mui/joy';
import {usePlaybackState, useProgress} from '../../model-hooks/transport-hooks';
import {useSong} from '../../model-hooks/song-hooks';
import {useSectionById} from '../../model-hooks/section-hooks';
import {PlayArrow, Repeat, Stop} from '@mui/icons-material';
import {
  exitLoopRequest,
  loopRequest,
  playRequest,
  stopRequest,
} from '../../api/request';
import {useCore} from '../../core/use-core';

export const Transport = () => {
  const playbackState = usePlaybackState();
  const progress = useProgress();
  const core = useCore();

  const playingSong = useSong(playbackState?.songId ?? '');
  const playingSection = useSectionById(playbackState?.sectionId ?? '');

  const play = () => {
    const request = playRequest();
    core.sendRequest(request);
  };

  const stop = () => {
    const request = stopRequest();
    core.sendRequest(request);
  };

  const toggleLoop = () => {
    const request = playbackState?.looping ? exitLoopRequest() : loopRequest();
    core.sendRequest(request);
  };

  return (
    <Card>
      <Stack spacing={2}>
        <LinearProgress
          determinate
          value={progress.sectionProgress * 100}
          sx={{color: 'orange'}}
        />
        <LinearProgress determinate value={progress.songProgress * 100} />

        <Stack direction="column" spacing={1} sx={{height: 72}}>
          <Typography level="title-lg">{playingSong?.name || ''}</Typography>
          <Typography level="title-md">{playingSection?.name || ''}</Typography>
        </Stack>

        <Metronome beat={progress.sectionBeat} blobCount={4} />

        <ButtonGroup>
          <IconButton
            onClick={play}
            color="primary"
            disabled={!playbackState?.playing}
            variant="outlined"
          >
            <PlayArrow />
          </IconButton>
          <IconButton onClick={stop} color="primary" variant="outlined">
            <Stop />
          </IconButton>
          <IconButton
            onClick={toggleLoop}
            color="primary"
            variant={playbackState?.looping ? 'solid' : 'outlined'}
            disabled={!playbackState?.playing}
          >
            <Repeat />
          </IconButton>
        </ButtonGroup>
      </Stack>
    </Card>
  );
};

const Metronome = ({beat, blobCount}: {beat: number; blobCount: number}) => {
  const indexes = Array.from({length: blobCount}, (_, i) => i);
  return (
    <Stack direction="row" spacing={1}>
      {indexes.map((beatIndex) => (
        <Box
          key={beatIndex}
          sx={{
            width: '8px',
            height: '8px',
            borderRadius: '4px',
            background:
              Math.floor(beat) % blobCount === beatIndex ? 'black' : 'grey',
          }}
        />
      ))}
    </Stack>
  );
};
