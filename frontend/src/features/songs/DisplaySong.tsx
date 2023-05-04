import React from 'react';
import {SongHeader} from './SongHeader';
import {Waveform} from '../waveforms/Waveform';
import {FiEdit2} from 'react-icons/fi';
import {Sections} from '../sections/Sections';
import {SecondaryButton} from '../../components/Button';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSelectedSongId, useSong} from './song-hooks';
import styles from './DisplaySong.module.css';

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
    <div className={styles.container}>
      <SongHeader selected={isSelected} name={song?.name || ''} />

      <div className={styles.waveform}>
        <Waveform sampleId={song?.sampleId} />

        {playbackState?.playing && playbackState?.songId === songId && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={'var(--primary)'}
          />
        )}
      </div>

      <Sections songId={songId} sectionIds={song?.sectionIds || []} />

      <div
        style={{
          display: 'flex',
          flexDirection: 'row-reverse',
          paddingRight: 16,
        }}
      >
        <SecondaryButton
          className={styles['edit-button']}
          onClick={() => setEditingSongId(songId)}
        >
          <FiEdit2 />
          <label>Edit</label>
        </SecondaryButton>
      </div>
    </div>
  );
};
