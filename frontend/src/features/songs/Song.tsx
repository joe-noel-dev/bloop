import {useCore} from '../core/use-core';
import {DisplaySong} from './DisplaySong';
import {SongCard} from './SongCard';
import {SongEditor} from './SongEditor';
import {selectSongRequest} from '../../api/request';
import {useSelectedSongId, useSong} from './song-hooks';

interface SongProps {
  songId: string;
  editEnabled: boolean;
}

export const Song = ({songId, editEnabled}: SongProps) => {
  const song = useSong(songId);
  const core = useCore();
  const selectedSongId = useSelectedSongId();

  const isSelected = (song && selectedSongId === song.id) || false;

  return (
    <SongCard
      isSelected={isSelected}
      onSelectSong={() => core?.sendRequest(selectSongRequest(songId))}
    >
      {!editEnabled && <DisplaySong songId={songId} />}
      {editEnabled && <SongEditor songId={songId} />}
    </SongCard>
  );
};
