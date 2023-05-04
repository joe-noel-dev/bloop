import React from 'react';
import styled from 'styled-components';
import {SongHeader} from './SongHeader';
import {Waveform} from '../waveforms/Waveform';
import {FiEdit2} from 'react-icons/fi';
import {MediumMain} from '../../typography/Typography';
import {Sections} from '../sections/Sections';
import {SecondaryButton} from '../../components/Button';
import {ProgressBar} from '../../components/ProgressBar';
import {appTheme} from '../theme';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSelectedSongId, useSong} from './song-hooks';

const WaveformContainer = styled.div`
  height: ${(props) => props.theme.units(10)};
  position: relative;
`;

const Container = styled.div`
  padding-bottom: ${(props) => props.theme.units(2)};
`;

interface Props {
  songId: string;
  setEditingSongId: (id: string) => void;
}

export const DisplaySong = ({songId, setEditingSongId}: Props) => {
  const song = useSong(songId);
  const selectedSongId = useSelectedSongId();
  const isSelected = (song && selectedSongId === song.id) || false;
  const progress = useProgress();
  const playbackState = usePlaybackState();

  return (
    <Container>
      <SongHeader selected={isSelected} name={song?.name || ''} />

      <WaveformContainer>
        <Waveform sampleId={song?.sampleId} />
        {playbackState?.playing && playbackState?.songId === songId && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={appTheme.colours.primary}
          />
        )}
      </WaveformContainer>

      <Sections songId={songId} sectionIds={song?.sectionIds || []} />

      <div
        style={{
          display: 'flex',
          flexDirection: 'row-reverse',
          paddingRight: 16,
        }}
      >
        <SecondaryButton onClick={() => setEditingSongId(songId)}>
          <FiEdit2 size={16} />
          <ButtonText>Edit</ButtonText>
        </SecondaryButton>
      </div>
    </Container>
  );
};

const ButtonText = styled.p`
  ${MediumMain};
`;
