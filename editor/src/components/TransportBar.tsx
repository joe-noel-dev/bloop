import {Box, IconButton, Typography} from '@mui/joy';
import {PlayArrow, Stop} from '@mui/icons-material';
import {useAppState} from '../state/AppState';
import {useSelectedSong, useSong} from '../model-hooks/song-hooks';
import {useSelectedSection} from '../model-hooks/section-hooks';
import {useSectionById} from '../model-hooks/section-hooks';
import {useDispatcher} from '../dispatcher/dispatcher';
import {playAction, stopAction} from '../dispatcher/action';
import {spacing, shadows, transitions} from '../theme';
import {INVALID_ID} from '../api/helpers';

export const TransportBar = () => {
  const {playbackState} = useAppState();
  const selectedSong = useSelectedSong();
  const selectedSection = useSelectedSection();
  const playingSong = useSong(playbackState?.songId || INVALID_ID);
  const playingSection = useSectionById(playbackState?.sectionId || INVALID_ID);
  const dispatch = useDispatcher();

  // Determine which song/section to display
  const displaySong = playbackState ? playingSong : selectedSong;
  const displaySection =
    playbackState && playingSection ? playingSection : selectedSection;

  const handlePlayStop = () => {
    if (playbackState) {
      dispatch(stopAction());
    } else {
      // Need both song and section to play
      if (selectedSong && selectedSection) {
        dispatch(playAction(selectedSong.id, selectedSection.id));
      }
    }
  };

  const canPlay = selectedSong && selectedSection;

  return (
    <Box
      sx={{
        position: 'fixed',
        bottom: 0,
        left: 0,
        right: 0,
        height: spacing.xxxxl,
        backgroundColor: 'background.surface',
        borderTop: '1px solid',
        borderTopColor: 'divider',
        boxShadow: shadows.level1,
        display: 'flex',
        alignItems: 'center',
        paddingX: spacing.md,
        gap: spacing.md,
        zIndex: 1000,
      }}
    >
      {/* Play/Stop Button */}
      <IconButton
        variant={playbackState ? 'solid' : 'soft'}
        color={playbackState ? 'primary' : 'neutral'}
        disabled={!canPlay}
        onClick={handlePlayStop}
        sx={{
          minWidth: spacing.xl,
          minHeight: spacing.xl,
          transition: transitions.normal,
        }}
      >
        {playbackState ? <Stop /> : <PlayArrow />}
      </IconButton>

      {/* Song and Section Info */}
      <Box sx={{flex: 1, minWidth: 0}}>
        <Typography
          level="title-sm"
          sx={{
            color: displaySong ? 'text.primary' : 'text.tertiary',
            fontWeight: 'md',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}
        >
          {displaySong?.name || 'No song selected'}
        </Typography>
        <Typography
          level="body-xs"
          sx={{
            color: displaySection ? 'text.secondary' : 'text.tertiary',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}
        >
          {displaySection?.name || 'No section selected'}
        </Typography>
      </Box>
    </Box>
  );
};
