import cloneDeep from 'lodash.clonedeep';
import React, {useState, forwardRef} from 'react';
import styled from 'styled-components';
import {SecondaryButton, WarningButton} from '../../components/Button';
import {MediumMain} from '../../typography/Typography';
import {useCore} from '../core/use-core';
import {SectionEditor} from '../sections/SectionEditor';
import {NameEditor} from '../../components/NameEditor';
import {useSelectedSongId, useSong, useSongs} from './song-hooks';
import {Sample} from '../samples/Sample';
import {FiCheck, FiPlus, FiTrash} from 'react-icons/fi';
import {horizontalGap, verticalGap} from '../../components/Gap';
import {
  addSampleRequest,
  addSectionRequest,
  beginUploadRequest,
  completeUploadRequest,
  removeSampleRequest,
  removeSectionRequest,
  removeSongRequest,
  updateSongRequest,
  uploadRequest,
} from '../../api/request';
import {v4 as uuidv4} from 'uuid';

interface Props {
  songId: string;
  setEditingSongId: (id: string) => void;
}

export const SongEditor = forwardRef<HTMLDivElement, Props>((props, ref) => {
  const song = useSong(props.songId);
  const core = useCore();
  const [editingSectionId, setEditingSectionId] = useState('');
  const selectedSongId = useSelectedSongId();
  const songs = useSongs();

  const isSelected = selectedSongId === props.songId;
  const isLastSong = songs?.length === 1;

  if (!song) {
    return <Container />;
  }

  const saveButton = () => {
    return (
      <SecondaryButton onClick={() => props.setEditingSongId('')}>
        <FiCheck size={16} />
        <ButtonText>Done</ButtonText>
      </SecondaryButton>
    );
  };

  const removeButton = () => {
    return (
      <WarningButton
        onClick={() => core?.sendRequest(removeSongRequest(props.songId))}
      >
        <FiTrash size={16} />
        <ButtonText>Remove Song</ButtonText>
      </WarningButton>
    );
  };

  const addSampleToSong = async (file: File) => {
    const uploadId = uuidv4();

    core?.sendRequest(beginUploadRequest(uploadId, file.name, 'wav'));
    await core?.waitForUploadAck(uploadId);

    const reader = new FileReader();

    reader.onload = async () => {
      const result = reader.result as ArrayBuffer;
      core?.sendRequest(uploadRequest(uploadId, result));
      await core?.waitForUploadAck(uploadId);

      core?.sendRequest(completeUploadRequest(uploadId));
      await core?.waitForUploadAck(uploadId);

      core?.sendRequest(addSampleRequest(song.id, uploadId));
    };

    reader.readAsArrayBuffer(file);
  };

  return (
    <Container ref={ref}>
      <NameRegion isSelected={isSelected}>
        <NameEditor
          name={song.name}
          onSave={(name) => {
            const newSong = cloneDeep(song!);
            newSong.name = name;
            core?.sendRequest(updateSongRequest(newSong));
          }}
          editable={true}
        />
      </NameRegion>
      <Sample
        editable={true}
        sampleId={song.sampleId}
        songId={props.songId}
        onFileSelected={(file) => addSampleToSong(file)}
        onRemoveRequested={() =>
          core?.sendRequest(removeSampleRequest(song.sampleId, song.id))
        }
      />
      <SectionRegion>
        {song.sectionIds.map((sectionId: string) => (
          <SectionEditor
            key={sectionId}
            sectionId={sectionId}
            sampleId={song.sampleId}
            editing={editingSectionId === sectionId}
            onRequestEdit={(shouldEdit) =>
              setEditingSectionId(shouldEdit ? sectionId : '')
            }
            canRemove={song.sectionIds.length > 1}
            onRequestRemove={() =>
              core?.sendRequest(removeSectionRequest(sectionId))
            }
          />
        ))}
        <AddSectionButton
          onClick={() => core?.sendRequest(addSectionRequest(song.id))}
        >
          <FiPlus size={16} />
          <ButtonText>Add Section</ButtonText>
        </AddSectionButton>
      </SectionRegion>
      <ButtonRegion>
        {!isLastSong && removeButton()}
        <div style={{flex: 1}} />
        {saveButton()}
      </ButtonRegion>
    </Container>
  );
});

const ButtonText = styled.p`
  ${MediumMain};
`;

const Container = styled.div`
  padding-bottom: ${(props) => props.theme.units(2)};

  display: flex;
  flex-direction: column;
`;

interface NameRegionProps {
  isSelected: boolean;
}

const NameRegion = styled.div<NameRegionProps>`
  padding: ${(props) => props.theme.units(2)};
  background: ${(props) =>
    props.isSelected
      ? props.theme.colours.primary
      : props.theme.colours.cardBackground};
  color: ${(props) =>
    props.isSelected
      ? props.theme.textColours.primary
      : props.theme.textColours.card};
`;

const ButtonRegion = styled.div`
  display: flex;

  padding: 0 ${(props) => props.theme.units(2)};
`;

const SectionRegion = styled.div`
  padding: 0 ${(props) => props.theme.units(2)};

  display: flex;
  flex-direction: column;
  ${(props) => verticalGap(props.theme.units(2))};

  margin: ${(props) => props.theme.units(2)} 0;
`;

const AddSectionButton = styled.button`
  height: ${(props) => props.theme.units(6)};
  padding: ${(props) => props.theme.units(2)};
  border: 1px solid ${(props) => props.theme.colours.cardLayer};
  borderradius: ${(props) => props.theme.borderRadius};

  display: flex;
  align-items: center;

  ${(props) => horizontalGap(props.theme.units(1))};

  display: flex;

  margin-right: auto;
`;
