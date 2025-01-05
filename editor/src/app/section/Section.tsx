import {
  ColorPaletteProp,
  Grid,
  IconButton,
  LinearProgress,
  Stack,
  Switch,
} from '@mui/joy';
import {
  useSectionById,
  useSelectedSectionId,
} from '../../model-hooks/section-hooks';
import {useCore} from '../../core/use-core';
import {
  playRequest,
  queueRequest,
  removeSectionRequest,
  selectSectionRequest,
  stopRequest,
  updateSectionRequest,
  updateSongRequest,
} from '../../api/request';
import {
  ArrowDownward,
  ArrowForward,
  ArrowUpward,
  Delete,
  PlayArrow,
  Stop,
} from '@mui/icons-material';
import {usePlaybackState, useProgress} from '../../model-hooks/transport-hooks';
import {columnSize, columns} from './TableInfo';
import isEqual from 'lodash.isequal';
import {getSectionBeatLength, Song} from '../../model/song';
import {useSong} from '../../model-hooks/song-hooks';
import {ClickToEdit} from '../../components/ClickToEdit';
import {Core} from '../../core/Core';

interface Props {
  songId: string;
  sectionId: string;
  requestUpdateDuration(sectionId: string, duration: number): void;
}

export const Section = ({songId, sectionId, requestUpdateDuration}: Props) => {
  const section = useSectionById(sectionId);
  const core = useCore();
  const song = useSong(songId);
  const selectedSectionId = useSelectedSectionId();
  const playbackState = usePlaybackState();
  const progress = useProgress();

  const duration = song ? getSectionBeatLength(song, sectionId) : 0;
  const isFirst = song?.sections.at(0)?.id === sectionId;
  const isLast = song?.sections.at(-1)?.id === sectionId;

  const isSelected = sectionId === selectedSectionId;
  const isPlaying =
    (playbackState?.playing && playbackState.sectionId === sectionId) ?? false;

  if (!section) {
    return <></>;
  }

  const enableLoop = (enable: boolean) => {
    const request = updateSectionRequest({
      ...section,
      loop: enable,
    });
    core.sendRequest(request);
  };

  const enableMetronome = (enable: boolean) => {
    const request = updateSectionRequest({
      ...section,
      metronome: enable,
    });
    core.sendRequest(request);
  };

  const select = () => {
    if (isSelected) {
      return;
    }

    const request = selectSectionRequest(sectionId);
    core.sendRequest(request);
  };

  const remove = () => {
    const request = removeSectionRequest(sectionId);
    core.sendRequest(request);
  };

  const submitName = (name: string) => {
    const newSection = {...section, name};

    if (isEqual(section, newSection)) {
      return;
    }

    const request = updateSectionRequest(newSection);
    core.sendRequest(request);
  };

  const submitStart = (value: string) => {
    const newStart = parseFloat(value);
    if (isNaN(newStart) || newStart === section.start) {
      console.warn('Invalid start: ', value);
      return;
    }

    const newSection = {...section, start: newStart};
    const request = updateSectionRequest(newSection);
    core.sendRequest(request);
  };

  const submitDuration = (value: string) => {
    const newDuration = parseFloat(value);
    if (isNaN(newDuration) || newDuration === duration) {
      console.warn('Invalid duration: ', value);
      return;
    }

    requestUpdateDuration(sectionId, newDuration);
  };

  const stop = () => {
    const stop = stopRequest();
    core.sendRequest(stop);
  };

  const play = () => {
    stop();
    select();

    const play = playRequest();
    core.sendRequest(play);
  };

  const queue = () => {
    const request = queueRequest(songId, sectionId);
    core.sendRequest(request);
  };

  return (
    <Grid container spacing={1}>
      {columns.map((name) => {
        switch (name) {
          case 'Play':
            return (
              <TransportCell
                key={name}
                isPlaying={isPlaying}
                progress={progress.sectionProgress}
                sectionBeat={progress.sectionBeat}
                onRequestPlay={play}
                onRequestStop={stop}
                onRequestQueue={queue}
              />
            );
          case 'Name':
            return (
              <NameCell key={name} value={section.name} onSubmit={submitName} />
            );
          case 'Start':
            return (
              <StartCell
                key={name}
                value={`${section.start}`}
                onChange={submitStart}
              />
            );
          case 'Duration':
            return (
              <DurationCell
                key={name}
                display={!isLast}
                value={`${duration}`}
                onChange={submitDuration}
              />
            );
          case 'Loop':
            return (
              <LoopCell key={name} loop={section.loop} onChange={enableLoop} />
            );
          case 'Metronome':
            return (
              <MetronomeCell
                key={name}
                metronome={section.metronome}
                onChange={enableMetronome}
              />
            );
          case 'Edit':
            return (
              <Grid xs={columnSize('Edit')} key={name}>
                <Stack direction="row" spacing={1} alignItems="center">
                  <EditButton
                    onClick={() => {
                      if (!song) {
                        return;
                      }

                      const delta = -1;
                      moveSectionIndex(sectionId, delta, song, core);
                    }}
                    color="primary"
                    disabled={isFirst}
                  >
                    <ArrowUpward />
                  </EditButton>

                  <EditButton
                    onClick={() => {
                      if (!song) {
                        return;
                      }

                      const delta = 1;
                      moveSectionIndex(sectionId, delta, song, core);
                    }}
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

const PlayButton = ({
  isPlaying,
  onRequestPlay,
}: {
  isPlaying: boolean;
  onRequestPlay: () => void;
}) =>
  !isPlaying && (
    <IconButton onClick={onRequestPlay}>
      <PlayArrow />
    </IconButton>
  );

const StopButton = ({
  isPlaying,
  onRequestStop,
}: {
  isPlaying: boolean;
  onRequestStop: () => void;
}) =>
  isPlaying && (
    <IconButton onClick={onRequestStop}>
      <Stop />
    </IconButton>
  );

const QueueButton = ({onRequestQueue}: {onRequestQueue: () => void}) => (
  <IconButton onClick={onRequestQueue}>
    <ArrowForward />
  </IconButton>
);

const Progress = ({
  isPlaying,
  progress,
}: {
  isPlaying: boolean;
  progress: number;
}) =>
  isPlaying && (
    <LinearProgress determinate value={progress * 100} sx={{maxWidth: 64}} />
  );

const TransportCell = ({
  isPlaying,
  progress,
  onRequestPlay,
  onRequestStop,
  onRequestQueue,
}: {
  isPlaying: boolean;
  progress: number;
  sectionBeat: number;
  onRequestPlay: () => void;
  onRequestStop: () => void;
  onRequestQueue: () => void;
}) => (
  <Grid xs={columnSize('Play')}>
    <Stack direction="row" spacing={1} alignItems="center">
      <PlayButton isPlaying={isPlaying} onRequestPlay={onRequestPlay} />
      <StopButton isPlaying={isPlaying} onRequestStop={onRequestStop} />
      <QueueButton onRequestQueue={onRequestQueue} />
      <Progress isPlaying={isPlaying} progress={progress} />
    </Stack>
  </Grid>
);

const NameCell = ({
  value,
  onSubmit,
}: {
  value: string;
  onSubmit: (name: string) => void;
}) => (
  <Grid xs={columnSize('Name')} sx={{display: 'flex', alignItems: 'center'}}>
    <ClickToEdit initialValue={value} onSave={onSubmit} />
  </Grid>
);

const StartCell = ({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) => (
  <Grid xs={columnSize('Start')} sx={{display: 'flex', alignItems: 'center'}}>
    <ClickToEdit initialValue={value} onSave={onChange} />
  </Grid>
);

const DurationCell = ({
  value,
  display,
  onChange,
}: {
  value: string;
  display: boolean;
  onChange: (value: string) => void;
}) => (
  <Grid
    xs={columnSize('Duration')}
    sx={{display: 'flex', alignItems: 'center'}}
  >
    {display && <ClickToEdit initialValue={value} onSave={onChange} />}
  </Grid>
);

const LoopCell = ({
  loop,
  onChange,
}: {
  loop: boolean;
  onChange: (loop: boolean) => void;
}) => (
  <Grid xs={columnSize('Loop')} sx={{display: 'flex', alignItems: 'center'}}>
    <Switch
      checked={loop}
      onChange={(event) => onChange(event.target.checked)}
    />
  </Grid>
);

const MetronomeCell = ({
  metronome,
  onChange,
}: {
  metronome: boolean;
  onChange: (metronome: boolean) => void;
}) => (
  <Grid
    xs={columnSize('Metronome')}
    sx={{display: 'flex', alignItems: 'center'}}
  >
    <Switch
      checked={metronome}
      onChange={(event) => onChange(event.target.checked)}
    />
  </Grid>
);

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

const moveSectionIndex = (
  sectionId: string,
  delta: number,
  song: Song,
  core: Core
) => {
  const currentIndex = song.sections.findIndex((s) => s.id === sectionId);
  if (currentIndex === -1) {
    console.warn(`Section not found: ${sectionId}`);
    return;
  }

  const newIndex = currentIndex + delta;
  if (newIndex < 0 || newIndex >= song.sections.length) {
    console.warn(`Section move out of range: ${sectionId} -> ${newIndex}`);
    return;
  }

  const startPositions = song.sections.map((s) => s.start);
  const newSong = {...song};
  const [movedSection] = newSong.sections.splice(currentIndex, 1);
  newSong.sections.splice(newIndex, 0, movedSection);

  // Maintain beat start positions
  newSong.sections.forEach((section, index) => {
    section.start = startPositions[index];
  });

  const request = updateSongRequest(newSong);
  core.sendRequest(request);
};
