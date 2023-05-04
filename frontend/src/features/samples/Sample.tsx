import React, {useEffect, useRef, useState} from 'react';
import {FiRepeat, FiTrash, FiUpload} from 'react-icons/fi';
import {Waveform} from '../waveforms/Waveform';
import {NameEditor} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import {cloneDeep} from 'lodash';
import {ProgressBar} from '../../components/ProgressBar';
import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import {updateSampleRequest} from '../../api/request';
import {useSampleWithId} from './sample-hooks';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import styles from './Sample.module.css';
import {SecondaryButton} from '../../components/Button';

interface SampleProps {
  editable: boolean;
  sampleId: string;
  songId: string;
  onFileSelected?(file: File): void;
  onRemoveRequested?(): void;
}

export const Sample = (props: SampleProps) => {
  const sample = useSampleWithId(props.sampleId);
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

  const onFileSelected = async () => {
    if (
      fileInputRef &&
      fileInputRef.current &&
      fileInputRef.current.files?.length
    ) {
      setUploading(true);

      const file = fileInputRef?.current?.files[0];

      if (props.onFileSelected) {
        props.onFileSelected(file);
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
        <Waveform sampleId={props.sampleId} />
        {playbackState?.playing && playbackState.songId === props.songId && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={'var(--primary)'}
          />
        )}
        {props.editable && (
          <input
            type="file"
            accept="audio/wav"
            onChange={onFileSelected}
            ref={fileInputRef}
            style={{display: 'none'}}
          />
        )}
        {props.editable && !sample && !uploading && (
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
        {props.editable && sample && (
          <button
            className={styles['remove-button']}
            onClick={() => {
              if (props.onRemoveRequested) {
                props.onRemoveRequested();
              }
            }}
          >
            <FiTrash size={16} />
            <p>Remove</p>
          </button>
        )}
        {props.editable && sample && (
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
          <p>Original tempo</p>
          <NameEditor
            onSave={(value) => onTempoChanged(value)}
            name={`${sample?.tempo.bpm}` || ''}
            editable={props.editable}
            inputType="number"
          ></NameEditor>
        </div>
      )}
    </div>
  );
};
