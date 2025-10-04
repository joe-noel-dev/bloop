import {ColorPaletteProp, Grid, IconButton, Stack, Switch} from '@mui/joy';
import {useSectionById} from '../../model-hooks/section-hooks';
import {
  ArrowDownward,
  ArrowUpward,
  Delete,
  PlayArrow,
  Stop,
} from '@mui/icons-material';
import {columnSize, columns} from './TableInfo';
import isEqual from 'lodash.isequal';
import {useSong} from '../../model-hooks/song-hooks';
import {ClickToEdit} from '../../components/ClickToEdit';
import {getSectionBeatLength, ID} from '../../api/helpers';
import {Section as ModelSection} from '../../api/bloop';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  moveSectionAction,
  playAction,
  removeSectionAction,
  stopAction,
  updateSectionAction,
} from '../../dispatcher/action';
import {useAppState} from '../../state/AppState';

interface Props {
  songId: ID;
  sectionId: ID;
  requestUpdateDuration(sectionId: ID, duration: number): void;
}

export const Section = ({songId, sectionId, requestUpdateDuration}: Props) => {
  const section = useSectionById(sectionId);
  const song = useSong(songId);
  const state = useAppState();
  const dispatch = useDispatcher();

  const duration = song ? getSectionBeatLength(song, sectionId) : 0;
  const isFirst = song?.sections.at(0)?.id === sectionId;
  const isLast = song?.sections.at(-1)?.id === sectionId;
  const sectionIndex =
    song?.sections.findIndex((section) => section.id.equals(sectionId)) ?? 0;

  if (!section) {
    return <></>;
  }

  const updateSection = (section: ModelSection) =>
    dispatch(updateSectionAction(songId, section));

  const enableLoop = (enable: boolean) =>
    updateSection({
      ...section,
      loop: enable,
    });

  const enableMetronome = (enable: boolean) =>
    updateSection({
      ...section,
      metronome: enable,
    });

  const remove = () => dispatch(removeSectionAction(songId, sectionId));

  const handlePlay = () => {
    dispatch(playAction(songId, sectionId));
  };

  const handleStop = () => {
    dispatch(stopAction());
  };

  const submitName = (name: string) => {
    const newSection = {...section, name};

    if (isEqual(section, newSection)) {
      return;
    }

    updateSection(newSection);
  };

  const submitStart = (value: string) => {
    const newStart = parseFloat(value);
    if (isNaN(newStart) || newStart === section.start) {
      console.warn('Invalid start: ', value);
      return;
    }

    const newSection = {...section, start: newStart};
    updateSection(newSection);
  };

  const submitDuration = (value: string) => {
    const newDuration = parseFloat(value);
    if (isNaN(newDuration) || newDuration === duration) {
      console.warn('Invalid duration: ', value);
      return;
    }

    requestUpdateDuration(sectionId, newDuration);
  };

  const isPlaying =
    state.playbackState &&
    state.playbackState.songId?.equals(songId) &&
    state.playbackState.sectionId?.equals(sectionId);

  return (
    <Grid container spacing={1} sx={{color: 'inherit'}}>
      {columns.map((name) => {
        switch (name) {
          case 'Transport': {
            return (
              <Grid
                key={name}
                xs={1}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                <Stack direction="row" spacing={0.5}>
                  {isPlaying ? (
                    <EditButton onClick={handleStop} color="neutral">
                      <Stop />
                    </EditButton>
                  ) : (
                    <EditButton onClick={handlePlay} color="success">
                      <PlayArrow />
                    </EditButton>
                  )}
                </Stack>
              </Grid>
            );
          }
          case 'Name':
            return (
              <Grid
                key={name}
                xs={columnSize('Name')}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                <ClickToEdit initialValue={section.name} onSave={submitName} />
              </Grid>
            );
          case 'Start':
            return (
              <Grid
                key={name}
                xs={columnSize('Start')}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                <ClickToEdit
                  initialValue={`${section.start}`}
                  onSave={submitStart}
                />
              </Grid>
            );
          case 'Duration':
            return (
              <Grid
                key={name}
                xs={columnSize('Duration')}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                {!isLast && (
                  <ClickToEdit
                    initialValue={`${duration}`}
                    onSave={submitDuration}
                  />
                )}
              </Grid>
            );
          case 'Loop':
            return (
              <Grid
                key={name}
                xs={columnSize('Loop')}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                <Switch
                  id={`loop-${sectionId}`}
                  checked={section.loop}
                  onChange={(event) => enableLoop(event.target.checked)}
                />
              </Grid>
            );
          case 'Metronome':
            return (
              <Grid
                key={name}
                xs={columnSize('Metronome')}
                sx={{display: 'flex', alignItems: 'center'}}
              >
                <Switch
                  id={`metronome-${sectionId}`}
                  checked={section.metronome}
                  onChange={(event) => enableMetronome(event.target.checked)}
                />
              </Grid>
            );
          case 'Edit':
            return (
              <Grid xs={columnSize('Edit')} key={name}>
                <Stack direction="row" spacing={1} alignItems="center">
                  <EditButton
                    onClick={() =>
                      dispatch(
                        moveSectionAction(
                          songId,
                          sectionIndex,
                          sectionIndex - 1
                        )
                      )
                    }
                    color="primary"
                    disabled={isFirst}
                  >
                    <ArrowUpward />
                  </EditButton>

                  <EditButton
                    onClick={() =>
                      dispatch(
                        moveSectionAction(
                          songId,
                          sectionIndex,
                          sectionIndex + 1
                        )
                      )
                    }
                    color="primary"
                    disabled={isLast}
                  >
                    <ArrowDownward />
                  </EditButton>

                  <EditButton onClick={remove} color="danger">
                    <Delete />
                  </EditButton>
                </Stack>
              </Grid>
            );
        }
      })}
    </Grid>
  );
};
const EditButton = ({
  onClick,
  disabled,
  color,
  children,
}: {
  onClick: () => void;
  disabled?: boolean;
  color?: ColorPaletteProp;
  children: React.ReactNode;
}) => (
  <IconButton
    variant="soft"
    color={color}
    size="sm"
    disabled={disabled}
    aria-label="Move section up"
    onClick={(event) => {
      onClick();
      event.stopPropagation();
    }}
  >
    {children}
  </IconButton>
);
