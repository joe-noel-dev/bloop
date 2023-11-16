import {useEffect, useRef, useState} from 'react';
import {FiRepeat, FiTrash, FiUpload} from 'react-icons/fi';
import {Waveform} from '../waveforms/Waveform';
import {useCore} from '../core/use-core';
import cloneDeep from 'lodash.clonedeep';
import {ProgressBar} from '../../components/ProgressBar';
import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import {updateSampleRequest} from '../../api/request';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import styles from './Sample.module.css';
import {SecondaryButton} from '../../components/Button';
import {Sample as ModelSample} from '../../model/sample';
import {Song} from '../../model/song';
import {EditText} from 'react-edit-text';

interface SampleProps {
  editable: boolean;
  sample?: ModelSample;
  song: Song;
  onFileSelected?(file: File): void;
  onRemoveRequested?(): void;
}

export const Sample = ({
  editable,
  sample,
  song,
  onFileSelected,
  onRemoveRequested,
}: SampleProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const core = useCore();
  const progress = useProgress();
  const playbackState = usePlaybackState();

  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    if (sample) {
      setUploading(false);
    }
  }, [sample]);

  const onSelected = async () => {
    if (
      fileInputRef &&
      fileInputRef.current &&
      fileInputRef.current.files?.length
    ) {
      setUploading(true);

      const file = fileInputRef?.current?.files[0];

      if (onFileSelected) {
        onFileSelected(file);
      }
    }
  };

  const onTempoChanged = (value: string) => {
    if (!sample) {
      return;
    }
    const tempo = parseFloat(value);
    const newSample = cloneDeep(sample);
    newSample.tempo.bpm = tempo;
    core?.sendRequest(updateSampleRequest(newSample));
  };

  return (
    <div className={styles['container']}>
      <div className={styles['waveform']}>
        <Waveform sample={sample} />

        {playbackState?.playing && playbackState.songId === song.id && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={'var(--primary)'}
          />
        )}
        {editable && (
          <input
            type="file"
            accept="audio/wav"
            onChange={onSelected}
            ref={fileInputRef}
            style={{display: 'none'}}
          />
        )}
        {editable && !sample && !uploading && (
          <div className={styles.upload}>
            <SecondaryButton onClick={() => fileInputRef.current?.click()}>
              <FiUpload size={16} />
              <p>Upload Audio</p>{' '}
            </SecondaryButton>
          </div>
        )}
        {!sample && uploading && (
          <div className={styles.spinner}>
            <IndeterminateSpinner />
          </div>
        )}
        {editable && sample && (
          <button
            className={styles['remove-button']}
            onClick={() => {
              if (onRemoveRequested) {
                onRemoveRequested();
              }
            }}
          >
            <FiTrash size={16} />
            <p>Remove</p>
          </button>
        )}
        {editable && sample && (
          <button
            className={styles['replace-button']}
            onClick={() => {
              fileInputRef.current?.click();
            }}
          >
            <FiRepeat size={16} />
            <p>Replace</p>
          </button>
        )}
      </div>
      {sample && (
        <div className={styles['sample-properties']}>
          <label>Sample tempo</label>
          <EditText
            defaultValue={`${sample.tempo.bpm}`}
            onSave={({value}) => onTempoChanged(value)}
            readonly={!editable}
            className={styles.tempo}
            inputClassName={styles.tempo}
            type="number"
          />
        </div>
      )}
    </div>
  );
};
