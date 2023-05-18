import {useState, forwardRef} from 'react';
import {useCore} from '../core/use-core';
import {SectionEditor} from '../sections/SectionEditor';
import {useSong} from './song-hooks';
import {Sample} from '../samples/Sample';
import {FiPlus} from 'react-icons/fi';
import {
  addSampleRequest,
  addSectionRequest,
  beginUploadRequest,
  completeUploadRequest,
  removeSampleRequest,
  removeSectionRequest,
  uploadRequest,
} from '../../api/request';
import {v4 as uuidv4} from 'uuid';
import styles from './SongEditor.module.css';

interface Props {
  songId: string;
}

export const SongEditor = forwardRef<HTMLDivElement, Props>((props, ref) => {
  const song = useSong(props.songId);
  const core = useCore();
  const [editingSectionId, setEditingSectionId] = useState('');

  if (!song) {
    return <div className={styles.container} />;
  }

  const addSampleToSong = async (file: File) => {
    const uploadId = uuidv4();

    core?.sendRequest(beginUploadRequest(uploadId, file.name, 'wav'));
    await core?.waitForUploadAck(uploadId);

    const reader = new FileReader();

    reader.onload = async () => {
      const result = reader.result as ArrayBuffer;

      const chunkSize = 1024 * 1024;
      let position = 0;
      while (position < result.byteLength) {
        const chunk = result.slice(position, position + chunkSize);
        core?.sendRequest(uploadRequest(uploadId, chunk));
        await core?.waitForUploadAck(uploadId);
        position += chunkSize;
      }

      core?.sendRequest(completeUploadRequest(uploadId));
      await core?.waitForUploadAck(uploadId);

      core?.sendRequest(addSampleRequest(song.id, uploadId));
    };

    reader.readAsArrayBuffer(file);
  };

  return (
    <div className={styles.container} ref={ref}>
      <Sample
        editable={true}
        sample={song.sample}
        songId={props.songId}
        onFileSelected={(file) => addSampleToSong(file)}
        onRemoveRequested={() =>
          core?.sendRequest(removeSampleRequest(song.id))
        }
      />

      <div className={styles['section-region']}>
        {song.sections.map((section) => (
          <SectionEditor
            key={section.id}
            section={section}
            song={song}
            sample={song.sample}
            editing={editingSectionId === section.id}
            onRequestEdit={(shouldEdit) =>
              setEditingSectionId(shouldEdit ? section.id : '')
            }
            canRemove={song.sections.length > 1}
            onRequestRemove={() =>
              core?.sendRequest(removeSectionRequest(section.id))
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
    </div>
  );
});
