import {Grid, IconButton, Input, Stack, Switch, Typography} from '@mui/joy';
import {
  useSectionById,
  useSelectedSectionId,
} from '../../model-hooks/section-hooks';
import {useCore} from '../../core/use-core';
import {
  playRequest,
  removeSectionRequest,
  selectSectionRequest,
  stopRequest,
  updateSectionRequest,
} from '../../api/request';
import _ from 'lodash';
import {
  Cancel,
  Check,
  Delete,
  Edit,
  PlayArrow,
  Stop,
} from '@mui/icons-material';
import {useEditingSection} from '../project/EditingSectionContext';
import {useState} from 'react';
import {usePlaybackState} from '../../model-hooks/transport-hooks';
import {columnSize, columns} from './TableInfo';

interface Props {
  sectionId: string;
}

export const Section = ({sectionId}: Props) => {
  const section = useSectionById(sectionId);
  const core = useCore();
  const selectedSectionId = useSelectedSectionId();
  const [editingSectionId, setEditingSectionId] = useEditingSection();
  const playbackState = usePlaybackState();

  const [editingName, setEditingName] = useState(section?.name ?? '');
  const [editingStart, setEditingStart] = useState(
    section?.start.toString() ?? ''
  );

  const isEditing = sectionId === editingSectionId;
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

    const request = updateSectionRequest(newSection);
    core.sendRequest(request);

    setEditingSectionId('');
  };

  const cancel = () => {
    setEditingName(section.name);
    setEditingSectionId('');
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

  return (
    <Grid container spacing={1}>
      {columns.map((name) => {
        switch (name) {
          case 'Play':
            return (
              <TransportCell
                key={name}
                isPlaying={isPlaying}
                onRequestPlay={play}
                onRequestStop={stop}
              />
            );
          case 'Name':
            return (
              <NameCell
                key={name}
                isEditing={isEditing}
                editingName={editingName}
                onEditingNameChange={setEditingName}
                sectionName={section.name}
              />
            );
          case 'Start':
            return (
              <StartCell
                key={name}
                isEditing={isEditing}
                editingStart={editingStart}
                onEditingStartChange={setEditingStart}
                sectionStart={section.start}
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
                  {!isEditing && (
                    <EditButton
                      onRequestEdit={() => setEditingSectionId(sectionId)}
                    />
                  )}

                  {isEditing && (
                    <>
                      <SubmitButton onSubmit={submit} />
                      <CancelButton onCancel={cancel} />
                      <RemoveButton onRemove={remove} />
                    </>
                  )}
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

const TransportCell = ({
  isPlaying,
  onRequestPlay,
  onRequestStop,
}: {
  isPlaying: boolean;
  onRequestPlay: () => void;
  onRequestStop: () => void;
}) => (
  <Grid xs={columnSize('Play')}>
    <PlayButton isPlaying={isPlaying} onRequestPlay={onRequestPlay} />
    <StopButton isPlaying={isPlaying} onRequestStop={onRequestStop} />
  </Grid>
);

const NameCell = ({
  isEditing,
  editingName,
  onEditingNameChange,
  sectionName,
}: {
  isEditing: boolean;
  editingName: string;
  onEditingNameChange: (name: string) => void;
  sectionName: string;
}) => (
  <Grid xs={columnSize('Name')}>
    {isEditing ? (
      <Input
        value={editingName}
        onChange={(event) => onEditingNameChange(event.target.value)}
      />
    ) : (
      <Typography>{sectionName}</Typography>
    )}
  </Grid>
);

const StartCell = ({
  isEditing,
  editingStart,
  onEditingStartChange,
  sectionStart,
}: {
  isEditing: boolean;
  editingStart: string;
  onEditingStartChange: (value: string) => void;
  sectionStart: number;
}) => (
  <Grid xs={columnSize('Start')}>
    {isEditing ? (
      <Input
        value={editingStart}
        onChange={(event) => onEditingStartChange(event.target.value)}
      />
    ) : (
      <Typography>{sectionStart}</Typography>
    )}
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

const EditButton = ({onRequestEdit}: {onRequestEdit: () => void}) => (
  <IconButton
    color="primary"
    size="sm"
    variant="soft"
    aria-label="Edit section"
    onClick={onRequestEdit}
  >
    <Edit />
  </IconButton>
);

const SubmitButton = ({onSubmit}: {onSubmit: () => void}) => (
  <IconButton
    variant="soft"
    color="success"
    size="sm"
    aria-label="Commit changes to section"
    onClick={onSubmit}
  >
    <Check />
  </IconButton>
);

const CancelButton = ({onCancel}: {onCancel: () => void}) => (
  <IconButton
    variant="soft"
    color="warning"
    size="sm"
    aria-label="Cancel changed to section"
    onClick={onCancel}
  >
    <Cancel />
  </IconButton>
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
