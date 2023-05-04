import cloneDeep from 'lodash.clonedeep';
import React, {useState, forwardRef} from 'react';
import {SecondaryButton, WarningButton} from '../../components/Button';
import {useCore} from '../core/use-core';
import {SectionEditor} from '../sections/SectionEditor';
import {NameEditor} from '../../components/NameEditor';
import {useSelectedSongId, useSong, useSongs} from './song-hooks';
import {Sample} from '../samples/Sample';
import {FiCheck, FiPlus, FiTrash} from 'react-icons/fi';
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
import styles from './SongEditor.module.css';
import {Spacer} from '../../components/Spacer';

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
    return <div className={styles.container} />;
  }

  const saveButton = () => {
    return (
      <SecondaryButton onClick={() => props.setEditingSongId('')}>
        <FiCheck />
        <label>Done</label>
      </SecondaryButton>
    );
  };

  const removeButton = () => {
    return (
      <WarningButton
        onClick={() => core?.sendRequest(removeSongRequest(props.songId))}
      >
        <FiTrash />
        <label>Remove Song</label>
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
    <div className={styles.container} ref={ref}>
      <div
        className={`${styles['name-region']} ${
          isSelected && styles['name-region-selected']
        }`}
      >
        <NameEditor
          name={song.name}
          onSave={(name) => {
            const newSong = cloneDeep(song!);
            newSong.name = name;
            core?.sendRequest(updateSongRequest(newSong));
          }}
          editable={true}
        />
      </div>
      <Sample
        editable={true}
        sampleId={song.sampleId}
        songId={props.songId}
        onFileSelected={(file) => addSampleToSong(file)}
        onRemoveRequested={() =>
          core?.sendRequest(removeSampleRequest(song.sampleId, song.id))
        }
      />

      <div className={styles['section-region']}>
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

        <button
          className={styles['add-section-button']}
          onClick={() => core?.sendRequest(addSectionRequest(song.id))}
        >
          <FiPlus />
          <label>Add Section</label>
        </button>
      </div>

      <div className={styles['button-region']}>
        {!isLastSong && removeButton()}
        <Spacer />
        {saveButton()}
      </div>
    </div>
  );
});
