import {LinearProgress, Stack, Typography} from '@mui/joy';
import {usePlaybackState, useProgress} from '../../model-hooks/transport-hooks';
import {useSong} from '../../model-hooks/song-hooks';
import {useSectionById} from '../../model-hooks/section-hooks';

export const Transport = () => {
  const playbackState = usePlaybackState();
  const progress = useProgress();

  const song = useSong(playbackState?.songId ?? '');
  const section = useSectionById(playbackState?.sectionId ?? '');

  return (
    <Stack spacing={2}>
      <LinearProgress determinate value={progress.songProgress * 100} />
      <Typography level="title-lg">{song?.name ?? '[No Song]'}</Typography>
      <Typography level="title-md">
        {section?.name ?? '[No Section]'}
      </Typography>
    </Stack>
  );
};
