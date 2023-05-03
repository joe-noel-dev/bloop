import React from 'react';
import styled from 'styled-components';
import {useCore} from '../core/use-core';
import {TempoEditor} from './TempoEditor';
import cloneDeep from 'lodash.clonedeep';
import {updateSongRequest} from '../../api/request';
import {useSong} from './song-hooks';

interface InfoPanelProps {
  songId: string;
}

const Container = styled.div`
  display: flex;

  > * {
    margin: 0.5rem 1rem 0 0;
  }

  & > :first-child {
    margin-left: 0;
  }
`;

export const InfoPanel = ({songId}: InfoPanelProps) => {
  const song = useSong(songId);
  const core = useCore();

  const setTempo = (value: number) => {
    if (isNaN(value)) {
      return;
    }

    if (!song || !core) {
      return;
    }

    if (30 <= value && value <= 300) {
      let newSong = cloneDeep(song);
      newSong.tempo.bpm = value;
      core.sendRequest(updateSongRequest(newSong));
    }
  };

  return (
    <Container>
      <TempoEditor
        value={song?.tempo.bpm || 120}
        onValueChanged={(newValue) => setTempo(newValue)}
      />
    </Container>
  );
};
