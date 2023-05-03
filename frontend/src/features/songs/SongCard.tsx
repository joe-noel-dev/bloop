import React from 'react';
import {FunctionComponent} from 'react';
import styled from 'styled-components';

interface ContainerProps {
  isSelected: boolean;
}

const Container = styled.div<ContainerProps>`
  background: ${(props) => props.theme.colours.cardBackground};

  border-radius: 10px;

  overflow: hidden;
`;

interface Props {
  isSelected: boolean;
  onSelectSong?(): void;
}

export const SongCard: FunctionComponent<Props> = (props) => {
  return (
    <Container onClick={props.onSelectSong} isSelected={props.isSelected}>
      {props.children}
    </Container>
  );
};
