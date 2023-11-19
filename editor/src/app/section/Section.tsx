import {IconButton, Input, Stack, Switch, Typography} from '@mui/joy';
import {
  useSectionById,
  useSelectedSectionId,
} from '../../model-hooks/section-hooks';
import {useCore} from '../../core/use-core';
import {
  removeSectionRequest,
  selectSectionRequest,
  updateSectionRequest,
} from '../../api/request';
import _ from 'lodash';
import {Cancel, Check, Delete, Edit} from '@mui/icons-material';
import {useEditingSection} from '../project/EditingSectionContext';
import {CSSProperties, useState} from 'react';

interface Props {
  sectionId: string;
}

export const Section = ({sectionId}: Props) => {
  const section = useSectionById(sectionId);
  const core = useCore();
  const selectedSectionId = useSelectedSectionId();
  const [editingSectionId, setEditingSectionId] = useEditingSection();

  const [editingName, setEditingName] = useState(section?.name);
  const [editingStart, setEditingStart] = useState(section?.start);

  const isEditing = sectionId === editingSectionId;
  const isSelected = sectionId === selectedSectionId;

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

  return (
    <tr
      onClick={select}
      style={
        isSelected
          ? ({
              '--TableCell-dataBackground':
                'var(--TableCell-selectedBackground)',
            } as CSSProperties)
          : {}
      }
    >
      <td>
        {isEditing ? (
          <Input
            value={editingName}
            onChange={(event) => setEditingName(event.target.value)}
          />
        ) : (
          <Typography>{section.name}</Typography>
        )}
      </td>
      <td>
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
      </td>
      <td>
        <Switch
          checked={section.loop}
          onChange={(event) => enableLoop(event.target.checked)}
        />
      </td>
      <td>
        <Switch
          checked={section.metronome}
          onChange={(event) => enableMetronome(event.target.checked)}
        />
      </td>
      <td>
        <Stack direction="row" spacing={1}>
          {!isEditing && (
            <IconButton
              color="primary"
              size="sm"
              variant="soft"
              aria-label="Edit section"
              onClick={() => setEditingSectionId(sectionId)}
            >
              <Edit />
            </IconButton>
          )}

          {isEditing && (
            <>
              <IconButton
                variant="soft"
                color="success"
                size="sm"
                aria-label="Commit changes to section"
                onClick={submit}
              >
                <Check />
              </IconButton>

              <IconButton
                variant="soft"
                color="warning"
                size="sm"
                aria-label="Cancel changed to section"
                onClick={cancel}
              >
                <Cancel />
              </IconButton>

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
            </>
          )}
        </Stack>
      </td>
    </tr>
  );
};
