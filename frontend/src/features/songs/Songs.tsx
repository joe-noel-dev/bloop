import React, {useState} from 'react';
import styled from 'styled-components';
import {AddSong} from './AddSong';
import {Song} from './Song';
import {useSongs} from './song-hooks';

const Scroller = styled.div`
  overflow-y: scroll;
  position: relative;

  scroll-snap-type: y proximity;
`;

interface ScrollContainerProps {
  height?: number;
}

const ScrollContainer = styled.div<ScrollContainerProps>`
  scroll-snap-align: none;

  overflow-y: hidden;

  margin: ${(props) => props.theme.units(4)};

  @media (min-width: 1000px) {
    width: calc(1000px - ${(props) => props.theme.units(4)});
    margin: ${(props) => props.theme.units(4)} auto;
  }
`;

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
    <Scroller>
      {displayItems?.map(({song, height}, index) => (
        <div key={song.id} style={scrollContainerStyle(height)}>
          <ScrollContainer>
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
          </ScrollContainer>
        </div>
      ))}
      <ScrollContainer>
        <AddSong />
      </ScrollContainer>
    </Scroller>
  );
};
