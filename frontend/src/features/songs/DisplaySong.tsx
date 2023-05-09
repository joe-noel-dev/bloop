import {Waveform} from '../waveforms/Waveform';
import {Sections} from '../sections/Sections';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSong} from './song-hooks';
import styles from './DisplaySong.module.css';

interface Props {
  songId: string;
}

export const DisplaySong = ({songId}: Props) => {
  const song = useSong(songId);
  const progress = useProgress();
  const playbackState = usePlaybackState();

  return (
    <div className={styles.container}>
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
    </div>
  );
};
