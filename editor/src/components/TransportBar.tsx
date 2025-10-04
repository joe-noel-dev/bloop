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
  const {playbackState, progress} = useAppState();
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
        justifyContent: 'center',
        paddingX: spacing.md,
        zIndex: 1000,
      }}
    >
      {/* Centered Controls Group */}
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          gap: spacing.lg,
          minWidth: '400px',
          maxWidth: '600px',
          justifyContent: 'center',
        }}
      >
        {/* Play/Stop Button */}
        <IconButton
          variant={playbackState ? 'solid' : 'soft'}
          color={playbackState ? 'primary' : 'neutral'}
          disabled={!canPlay}
          onClick={handlePlayStop}
          sx={{
            width: spacing.xxl,
            height: spacing.xxl,
            minWidth: spacing.xxl,
            minHeight: spacing.xxl,
            transition: transitions.normal,
            flexShrink: 0,
          }}
        >
          {playbackState ? <Stop /> : <PlayArrow />}
        </IconButton>

        {/* Song and Section Info - Centered */}
        <Box
          sx={{
            flex: 1,
            minWidth: 0,
            textAlign: 'center',
            maxWidth: '300px',
          }}
        >
          <Typography
            level="title-sm"
            sx={{
              color: displaySong ? 'text.primary' : 'text.tertiary',
              fontWeight: 'md',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
              lineHeight: 1.2,
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
              lineHeight: 1.2,
            }}
          >
            {displaySection?.name || 'No section selected'}
          </Typography>
        </Box>

        {/* Progress Info - Fixed Width */}
        <Box
          sx={{
            width: '80px',
            minWidth: '80px',
            display: 'flex',
            flexDirection: 'row',
            alignItems: 'center',
            justifyContent: 'space-around',
            opacity: playbackState ? 1 : 0,
            transition: transitions.normal,
            flexShrink: 0,
            height: '100%',
          }}
        >
          {/* Section Beat */}
          <Box
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <Typography
              level="body-xs"
              sx={{
                color: 'text.tertiary',
                fontSize: '8px',
                textTransform: 'uppercase',
                letterSpacing: '0.3px',
                lineHeight: 1,
                marginBottom: '1px',
              }}
            >
              Sec
            </Typography>
            <Typography
              level="body-sm"
              sx={{
                color: 'text.primary',
                fontFamily: 'monospace',
                fontWeight: 'bold',
                lineHeight: 1,
              }}
            >
              {Math.floor(progress?.sectionBeat ?? 0)}
            </Typography>
          </Box>

          {/* Song Beat */}
          <Box
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <Typography
              level="body-xs"
              sx={{
                color: 'text.tertiary',
                fontSize: '8px',
                textTransform: 'uppercase',
                letterSpacing: '0.3px',
                lineHeight: 1,
                marginBottom: '1px',
              }}
            >
              Song
            </Typography>
            <Typography
              level="body-sm"
              sx={{
                color: 'text.secondary',
                fontFamily: 'monospace',
                fontWeight: 'bold',
                lineHeight: 1,
              }}
            >
              {Math.floor(progress?.songBeat ?? 0)}
            </Typography>
          </Box>
        </Box>
      </Box>
    </Box>
  );
};
