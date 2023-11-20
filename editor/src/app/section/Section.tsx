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
import {ColumnName, columnSize, columns} from './TableInfo';

interface Props {
  sectionId: string;
}

export const Section = ({sectionId}: Props) => {
  const section = useSectionById(sectionId);
  const core = useCore();
  const selectedSectionId = useSelectedSectionId();
  const [editingSectionId, setEditingSectionId] = useEditingSection();
  const playbackState = usePlaybackState();

  const [editingName, setEditingName] = useState(section?.name);
  const [editingStart, setEditingStart] = useState(section?.start);

  const isEditing = sectionId === editingSectionId;
  const isSelected = sectionId === selectedSectionId;
  const isPlaying =
    playbackState?.playing && playbackState.sectionId === sectionId;

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

    if (editingStart !== undefined) {
      newSection.start = editingStart;
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

  const PlayButton = () =>
    !isPlaying && (
      <IconButton onClick={play}>
        <PlayArrow />
      </IconButton>
    );

  const StopButton = () =>
    isPlaying && (
      <IconButton onClick={stop}>
        <Stop />
      </IconButton>
    );

  const TransportCell = () => (
    <Grid xs={columnSize('Play')}>
      <PlayButton />
      <StopButton />
    </Grid>
  );

  const NameCell = () => (
    <Grid xs={columnSize('Name')}>
      {isEditing ? (
        <Input
          value={editingName}
          onChange={(event) => setEditingName(event.target.value)}
        />
      ) : (
        <Typography>{section.name}</Typography>
      )}
    </Grid>
  );

  const StartCell = () => (
    <Grid xs={columnSize('Start')}>
      {isEditing ? (
        <Input
          value={editingStart}
          onChange={(event) => {
            const value = parseInt(event.target.value);
            if (!isNaN(value)) {
              setEditingStart(value);
            } else {
              setEditingStart(0);
            }
          }}
        />
      ) : (
        <Typography>{section.start}</Typography>
      )}
    </Grid>
  );

  const LoopCell = () => (
    <Grid xs={columnSize('Loop')}>
      <Switch
        checked={section.loop}
        onChange={(event) => enableLoop(event.target.checked)}
      />
    </Grid>
  );

  const MetronomeCell = () => (
    <Grid xs={columnSize('Metronome')}>
      <Switch
        checked={section.metronome}
        onChange={(event) => enableMetronome(event.target.checked)}
      />
    </Grid>
  );

  const EditButton = () => (
    <IconButton
      color="primary"
      size="sm"
      variant="soft"
      aria-label="Edit section"
      onClick={() => setEditingSectionId(sectionId)}
    >
      <Edit />
    </IconButton>
  );

  const SubmitButton = () => (
    <IconButton
      variant="soft"
      color="success"
      size="sm"
      aria-label="Commit changes to section"
      onClick={submit}
    >
      <Check />
    </IconButton>
  );

  const CancelButton = () => (
    <IconButton
      variant="soft"
      color="warning"
      size="sm"
      aria-label="Cancel changed to section"
      onClick={cancel}
    >
      <Cancel />
    </IconButton>
  );

  const RemoveButton = () => (
    <IconButton
      variant="soft"
      color="danger"
      size="sm"
      aria-label="Remove section"
      onClick={(event) => {
        remove();
        event.stopPropagation();
      }}
    >
      <Delete />
    </IconButton>
  );

  const ButtonsCell = () => (
    <Grid xs={columnSize('Edit')}>
      <Stack direction="row" spacing={1}>
        {!isEditing && <EditButton />}

        {isEditing && (
          <>
            <SubmitButton />
            <CancelButton />
            <RemoveButton />
          </>
        )}
      </Stack>
    </Grid>
  );

  const Cell = ({name}: {name: ColumnName}) => {
    switch (name) {
      case 'Play':
        return <TransportCell />;
      case 'Name':
        return <NameCell />;
      case 'Start':
        return <StartCell />;
      case 'Loop':
        return <LoopCell />;
      case 'Metronome':
        return <MetronomeCell />;
      case 'Edit':
        return <ButtonsCell />;
    }
  };

  return (
    <Grid container spacing={1}>
      {columns.map((name) => (
        <Cell name={name} key={name} />
      ))}
    </Grid>
  );
};
