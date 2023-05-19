import {Song} from './Song';
import {useSongs} from './song-hooks';
import styles from './Songs.module.css';
import {SongHeader} from './SongHeader';
import {FiPlus} from 'react-icons/fi';
import {useCore} from '../core/use-core';
import {addSongRequest} from '../../api/request';
import {SecondaryButton} from '../../components/Button';

interface Props {
  editEnabled: boolean;
}

export const Songs = ({editEnabled}: Props) => {
  const songs = useSongs();
  const core = useCore();

  return (
    <div className={styles.container}>
      {songs?.map((song) => (
        <div key={song.id} className={styles.song}>
          <SongHeader songId={song.id} editEnabled={editEnabled} />
          <Song song={song} editEnabled={editEnabled} />
        </div>
      ))}
      {editEnabled && (
        <SecondaryButton
          className={styles['add-song-button']}
          onClick={() => core.sendRequest(addSongRequest())}
        >
          <FiPlus />
          <label>Add Song</label>
        </SecondaryButton>
      )}
    </div>
  );
};
