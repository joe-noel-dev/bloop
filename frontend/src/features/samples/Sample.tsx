import React, {useEffect, useRef, useState} from 'react';
import {FiRepeat, FiTrash, FiUpload} from 'react-icons/fi';
import styled from 'styled-components';
import {Waveform} from '../waveforms/Waveform';
import {MediumMain} from '../../typography/Typography';
import {SecondaryButton} from '../../components/Button';
import {NameEditor} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import {cloneDeep} from 'lodash';
import {ProgressBar} from '../../components/ProgressBar';
import {appTheme} from '../theme';
import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import {Centred} from '../../components/Centred';
import {horizontalGap, verticalGap} from '../../components/Gap';
import {updateSampleRequest} from '../../api/request';
import {useSampleWithId} from './sample-hooks';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';

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
    <Container>
      <WaveformContainer>
        <Waveform sampleId={props.sampleId} />
        {playbackState?.playing && playbackState.songId === props.songId && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={appTheme.colours.primary}
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
          <UploadButton onClick={() => fileInputRef.current?.click()}>
            <FiUpload size={16} />
            <ButtonText>Upload Audio</ButtonText>
          </UploadButton>
        )}
        {!sample && uploading && (
          <Centred>
            <IndeterminateSpinner />
          </Centred>
        )}
        {props.editable && sample && (
          <RemoveButton
            onClick={() => {
              if (props.onRemoveRequested) props.onRemoveRequested();
            }}
          >
            <FiTrash size={16} />
            <ButtonText>Remove</ButtonText>
          </RemoveButton>
        )}
        {props.editable && sample && (
          <ReplaceButton
            onClick={() => {
              fileInputRef.current?.click();
            }}
          >
            <FiRepeat size={16} />
            <ButtonText>Replace</ButtonText>
          </ReplaceButton>
        )}
      </WaveformContainer>
      {sample && (
        <SampleProperties>
          <ButtonText>Original tempo</ButtonText>
          <NameEditor
            onSave={(value) => onTempoChanged(value)}
            name={`${sample?.tempo.bpm}` || ''}
            editable={props.editable}
            inputType="number"
          ></NameEditor>
        </SampleProperties>
      )}
    </Container>
  );
};

const ButtonText = styled.p`
  ${MediumMain}
`;

const Container = styled.div``;

const SampleProperties = styled.div`
  padding: ${(props) => props.theme.units(2)};
  padding-bottom: 0;

  display: flex;
  flex-direction: column;

  ${(props) => verticalGap(props.theme.units(1))};
`;

const WaveformContainer = styled.div`
  position: relative;
  height: ${(props) => props.theme.units(20)};
`;

const UploadButton = styled(SecondaryButton)`
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
`;

const OverlayButton = styled.button`
  display: flex;

  ${(props) => horizontalGap(props.theme.units(1))};

  padding: ${(props) => props.theme.units(2)};
  background: ${(props) => props.theme.colours.primaryLight}B0;
  color: ${(props) => props.theme.textColours.primaryLight};

  :active {
    background: ${(props) => props.theme.colours.primaryLight};
  }
`;

const RemoveButton = styled(OverlayButton)`
  position: absolute;
  right: 0px;
  bottom: 0px;
  border-top-left-radius: ${(props) => props.theme.units(2)};
`;

const ReplaceButton = styled(OverlayButton)`
  position: absolute;
  top: 0px;
  left: 0px;
  border-bottom-right-radius: ${(props) => props.theme.units(2)};
`;
