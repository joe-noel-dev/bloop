import React, {useState} from 'react';
import {AddSong} from './AddSong';
import {Song} from './Song';
import {useSongs} from './song-hooks';
import styles from './Songs.module.css';

const scrollContainerStyle = (height?: number): React.CSSProperties => ({
  height: height ? height + 'px' : 'auto',
  transition: 'height 0.2s ease-out',
});

export const Songs = () => {
  const songs = useSongs();
  const [heights, setHeights] = useState<Array<number>>(
    new Array(songs?.length).fill(0)
  );
  const [editingSongId, setEditingSongId] = useState('');

  const displayItems = songs?.map((song, index) => ({
    song,
    height: heights[index],
  }));

  return (
    <div className={styles.scroller}>
      {displayItems?.map(({song, height}, index) => (
        <div key={song.id} style={scrollContainerStyle(height)}>
          <div className={styles['scroll-container']}>
            <Song
              songId={song.id}
              editingSongId={editingSongId}
              setEditingSongId={setEditingSongId}
              onHeightChange={(height) => {
                const newHeights = [...heights];
                newHeights[index] = height;
                setHeights(newHeights);
              }}
            />
          </div>
        </div>
      ))}

      <div className={styles['scroll-container']}>
        <AddSong />
      </div>
    </div>
  );
};
