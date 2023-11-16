import {Waveform} from '../waveforms/Waveform';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import styles from './DisplaySong.module.css';
import {SectionOverview} from '../sections/SectionOverview';
import {Song} from '../../model/song';

interface Props {
  song: Song;
}

export const DisplaySong = ({song}: Props) => {
  const progress = useProgress();
  const playbackState = usePlaybackState();

  return (
    <div className={styles.container}>
      <div className={styles.waveform}>
        <Waveform sample={song?.sample} />

        {playbackState?.playing && playbackState?.songId === song.id && (
          <ProgressBar
            progress={progress?.songProgress || 0}
            colour={'var(--primary)'}
          />
        )}
      </div>

      <SectionOverview song={song} />
    </div>
  );
};
