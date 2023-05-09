import {PopupMenu} from '../menu/PopupMenu';
import styles from './SongHeader.module.css';
import {FiMoreHorizontal} from 'react-icons/fi';
import {Spacer} from '../../components/Spacer';
import {useCore} from '../core/use-core';
import {useSelectedSongId, useSong} from './song-hooks';
import {
  removeSongRequest,
  selectSongRequest,
  updateSongRequest,
} from '../../api/request';
import {NameEditor} from '../../components/NameEditor';
import cloneDeep from 'lodash.clonedeep';

interface SongHeaderProps {
  songId: string;
  editEnabled: boolean;
}
export const SongHeader = ({songId, editEnabled}: SongHeaderProps) => {
  const core = useCore();
  const song = useSong(songId);
  const selectedSongId = useSelectedSongId();
  const isSelected = song?.id === selectedSongId;

  const updateSongName = (name: string) => {
    if (!song) {
      return;
    }

    const newSong = cloneDeep(song);
    newSong.name = name;
    const request = updateSongRequest(newSong);
    core.sendRequest(request);
  };

  const selectSong = () => core.sendRequest(selectSongRequest(song?.id ?? ''));

  return (
    <div
      className={`${styles.container} ${
        isSelected && styles['container-selected']
      }`}
      onClick={selectSong}
    >
      <NameEditor
        onSave={updateSongName}
        name={song?.name ?? ''}
        editable={editEnabled}
        textClassName={styles.name}
      />

      <Spacer />

      {editEnabled && (
        <>
          <PopupMenu
            menuItems={[
              {
                title: 'Remove Song',
                onClick: () => core?.sendRequest(removeSongRequest(songId)),
              },
            ]}
          >
            <button>
              <FiMoreHorizontal />
            </button>
          </PopupMenu>
        </>
      )}
    </div>
  );
};
