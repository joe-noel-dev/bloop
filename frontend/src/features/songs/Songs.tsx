import {Song} from './Song';
import {useSongs} from './song-hooks';
import styles from './Songs.module.css';
import {SongHeader} from './SongHeader';

interface Props {
  editEnabled: boolean;
}

export const Songs = ({editEnabled}: Props) => {
  const songs = useSongs();

  return (
    <div className={styles.container}>
      {songs?.map((song) => (
        <div key={song.id} className={styles.song}>
          <SongHeader songId={song.id} editEnabled={editEnabled} />
          <Song songId={song.id} editEnabled={editEnabled} />
        </div>
      ))}
    </div>
  );
};
