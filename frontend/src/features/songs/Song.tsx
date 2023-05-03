import {useCore} from '../core/use-core';
import {DisplaySong} from './DisplaySong';
import {SongCard} from './SongCard';
import {SongEditor} from './SongEditor';
import Measure from 'react-measure';
import {selectSongRequest} from '../../api/request';
import {useSelectedSongId, useSong} from './song-hooks';

interface SongProps {
  songId: string;
  editingSongId: string;
  setEditingSongId: (id: string) => void;
  onHeightChange(height: number): void;
}

export const Song = ({
  songId,
  editingSongId,
  setEditingSongId,
  onHeightChange,
}: SongProps) => {
  const song = useSong(songId);
  const core = useCore();
  const selectedSongId = useSelectedSongId();

  const isSelected = (song && selectedSongId === song.id) || false;
  const isEditing = (song && editingSongId === song.id) || false;

  return (
    <Measure
      bounds
      onResize={(contentRect) =>
        onHeightChange(contentRect.bounds?.height || 0)
      }
    >
      {({measureRef}) => (
        <div ref={measureRef}>
          <SongCard
            isSelected={isSelected}
            onSelectSong={() => core?.sendRequest(selectSongRequest(songId))}
          >
            {!isEditing && (
              <DisplaySong
                songId={songId}
                setEditingSongId={setEditingSongId}
              />
            )}
            {isEditing && (
              <SongEditor songId={songId} setEditingSongId={setEditingSongId} />
            )}
          </SongCard>
        </div>
      )}
    </Measure>
  );
};
