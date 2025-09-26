import {Box, Stack, Typography, Button} from '@mui/joy';
import {Add} from '@mui/icons-material';
import {GanttSection} from './GanttSection';
import {getSectionBeatLength, getSampleBeatLength, ID} from '../../api/helpers';
import {Song as ModelSong} from '../../api/bloop';

interface GanttViewProps {
  song: ModelSong;
  requestAdd: () => void;
  requestUpdateSectionDuration: (sectionId: ID, duration: number) => void;
  onSectionStartChange: (sectionId: ID, newStart: number) => void;
}

const TIMELINE_HEIGHT = 80;
const RULER_HEIGHT = 30;
const SECTION_TRACK_HEIGHT = 60;

export const GanttView = ({
  song,
  requestAdd,
  requestUpdateSectionDuration,
  onSectionStartChange,
}: GanttViewProps) => {
  // Calculate total duration of the song
  const totalDuration = song.sample ? getSampleBeatLength(song.sample) : 
    song.sections.length > 0 ? 
      Math.max(...song.sections.map(section => 
        section.start + getSectionBeatLength(song, section.id)
      )) : 100;

  // Generate ruler marks
  const generateRulerMarks = () => {
    const marks = [];
    const majorInterval = Math.max(1, Math.floor(totalDuration / 20)); // About 20 major marks
    const minorInterval = majorInterval / 4;

    for (let i = 0; i <= totalDuration; i += minorInterval) {
      const isMajor = i % majorInterval === 0;
      const leftPercent = (i / totalDuration) * 100;
      
      marks.push(
        <Box
          key={i}
          sx={{
            position: 'absolute',
            left: `${leftPercent}%`,
            top: 0,
            bottom: 0,
            width: 1,
            backgroundColor: isMajor ? 'neutral.500' : 'neutral.300',
            zIndex: 1,
          }}
        >
          {isMajor && (
            <Typography
              level="body-xs"
              sx={{
                position: 'absolute',
                top: -20,
                left: -10,
                width: 20,
                textAlign: 'center',
                fontSize: '10px',
                color: 'neutral.600',
              }}
            >
              {Math.round(i * 10) / 10}
            </Typography>
          )}
        </Box>
      );
    }
    return marks;
  };

  return (
    <Box
      sx={{
        backgroundColor: 'background.body',
        borderRadius: 'lg',
        padding: 3,
        border: '1px solid',
        borderColor: 'neutral.200',
      }}
    >
      <Stack spacing={2}>
        {/* Header */}
        <Stack direction="row" justifyContent="space-between" alignItems="center">
          <Typography level="title-md" fontWeight="bold">
            Sections Timeline
          </Typography>
          <Button
            startDecorator={<Add />}
            onClick={requestAdd}
            variant="soft"
            color="primary"
            size="sm"
          >
            Add Section
          </Button>
        </Stack>

        {/* Timeline container */}
        <Box
          sx={{
            position: 'relative',
            height: TIMELINE_HEIGHT + song.sections.length * (SECTION_TRACK_HEIGHT + 10),
            backgroundColor: 'background.surface',
            borderRadius: 'md',
            border: '1px solid',
            borderColor: 'neutral.200',
            overflow: 'hidden',
          }}
        >
          {/* Ruler */}
          <Box
            sx={{
              position: 'absolute',
              top: 0,
              left: 0,
              right: 0,
              height: RULER_HEIGHT,
              backgroundColor: 'background.level1',
              borderBottom: '1px solid',
              borderColor: 'neutral.200',
            }}
          >
            {generateRulerMarks()}
          </Box>

          {/* Sections track */}
          <Box
            sx={{
              position: 'absolute',
              top: RULER_HEIGHT,
              left: 0,
              right: 0,
              bottom: 0,
              paddingX: 2,
              paddingY: 1,
            }}
          >
            {song.sections.map((section) => {
              const duration = getSectionBeatLength(song, section.id);
              return (
                <Box
                  key={section.id.toString()}
                  sx={{
                    position: 'relative',
                    height: SECTION_TRACK_HEIGHT,
                    marginBottom: 1,
                    '&:not(:last-child)': {
                      borderBottom: '1px solid',
                      borderColor: 'neutral.100',
                    },
                  }}
                >
                  <GanttSection
                    songId={song.id}
                    sectionId={section.id}
                    startTime={section.start}
                    duration={duration}
                    totalDuration={totalDuration}
                    requestUpdateDuration={requestUpdateSectionDuration}
                    onSectionStartChange={onSectionStartChange}
                  />
                </Box>
              );
            })}
          </Box>

          {/* Empty state */}
          {song.sections.length === 0 && (
            <Box
              sx={{
                position: 'absolute',
                top: RULER_HEIGHT,
                left: 0,
                right: 0,
                bottom: 0,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                flexDirection: 'column',
                gap: 2,
              }}
            >
              <Typography level="body-md" color="neutral">
                No sections yet
              </Typography>
              <Button
                startDecorator={<Add />}
                onClick={requestAdd}
                variant="outlined"
                color="primary"
                size="sm"
              >
                Add First Section
              </Button>
            </Box>
          )}
        </Box>
      </Stack>
    </Box>
  );
};