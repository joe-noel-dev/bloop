import {useCore} from '../core/use-core';
import {DisplaySong} from './DisplaySong';
import {SongCard} from './SongCard';
import {SongEditor} from './SongEditor';
import {selectSongRequest} from '../../api/request';
import {useSelectedSongId} from './song-hooks';
import {Song as ModelSong} from '../../model/song';

interface SongProps {
  song: ModelSong;
  editEnabled: boolean;
}

export const Song = ({song, editEnabled}: SongProps) => {
  const core = useCore();
  const selectedSongId = useSelectedSongId();

  const isSelected = (song && selectedSongId === song.id) || false;

  return (
    <SongCard
      isSelected={isSelected}
      onSelectSong={() => core?.sendRequest(selectSongRequest(song.id))}
    >
      {!editEnabled && <DisplaySong song={song} />}
      {editEnabled && <SongEditor song={song} />}
    </SongCard>
  );
};
