import {
  CircularProgress,
  Grid,
  IconButton,
  Input,
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
} from '../../api/request';
import {ArrowForward, Delete, PlayArrow, Stop} from '@mui/icons-material';
import {useState} from 'react';
import {usePlaybackState, useProgress} from '../../model-hooks/transport-hooks';
import {columnSize, columns} from './TableInfo';
import isEqual from 'lodash.isequal';

interface Props {
  songId: string;
  sectionId: string;
}

export const Section = ({songId, sectionId}: Props) => {
  const section = useSectionById(sectionId);
  const core = useCore();
  const selectedSectionId = useSelectedSectionId();
  const playbackState = usePlaybackState();
  const progress = useProgress();

  const [editingName, setEditingName] = useState(section?.name ?? '');
  const [editingStart, setEditingStart] = useState(
    section?.start.toString() ?? ''
  );

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

  const submit = () => {
    const newSection = {...section};

    if (editingName !== undefined) {
      newSection.name = editingName;
    }

    const newStart = parseFloat(editingStart);
    if (!isNaN(newStart)) {
      newSection.start = newStart;
    }

    if (isEqual(section, newSection)) {
      return;
    }

    const request = updateSectionRequest(newSection);
    core.sendRequest(request);
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
              <NameCell
                key={name}
                value={editingName}
                onChange={setEditingName}
                onSubmit={submit}
              />
            );
          case 'Start':
            return (
              <StartCell
                key={name}
                value={editingStart}
                onChange={setEditingStart}
                onSubmit={submit}
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
                <Stack direction="row" spacing={1}>
                  <RemoveButton onRemove={remove} />
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
    <CircularProgress size="sm" determinate value={progress * 100} />
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
  <Grid xs={columnSize('Play')} alignItems="center">
    <Stack direction="row" spacing={1}>
      <PlayButton isPlaying={isPlaying} onRequestPlay={onRequestPlay} />
      <StopButton isPlaying={isPlaying} onRequestStop={onRequestStop} />
      <QueueButton onRequestQueue={onRequestQueue} />
      <Progress isPlaying={isPlaying} progress={progress} />
    </Stack>
  </Grid>
);

const NameCell = ({
  value,
  onChange,
  onSubmit,
}: {
  value: string;
  onChange: (name: string) => void;
  onSubmit: () => void;
}) => (
  <Grid xs={columnSize('Name')}>
    <Input
      value={value}
      onBlur={onSubmit}
      onChange={(event) => onChange(event.target.value)}
    />
  </Grid>
);

const StartCell = ({
  value,
  onChange,
  onSubmit,
}: {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
}) => (
  <Grid xs={columnSize('Start')}>
    <Input
      value={value}
      onChange={(event) => onChange(event.target.value)}
      onBlur={onSubmit}
    />
  </Grid>
);

const LoopCell = ({
  loop,
  onChange,
}: {
  loop: boolean;
  onChange: (loop: boolean) => void;
}) => (
  <Grid xs={columnSize('Loop')}>
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
  <Grid xs={columnSize('Metronome')}>
    <Switch
      checked={metronome}
      onChange={(event) => onChange(event.target.checked)}
    />
  </Grid>
);

const RemoveButton = ({onRemove}: {onRemove: () => void}) => (
  <IconButton
    variant="soft"
    color="danger"
    size="sm"
    aria-label="Remove section"
    onClick={(event) => {
      onRemove();
      event.stopPropagation();
    }}
  >
    <Delete />
  </IconButton>
);
