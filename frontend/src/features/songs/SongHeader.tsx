import React from 'react';
import styled from 'styled-components';
import {LargeMain} from '../../typography/Typography';

interface SongHeaderProps {
  name: string;
  selected: boolean;
}

interface ContainerProps {
  selected: boolean;
}

const Container = styled.div<ContainerProps>`
  background: ${(props) =>
    props.selected ? props.theme.colours.primary : 'rgba(0,0,0,0.2)'};
  color: ${(props) => props.theme.textColours.primary};

  height: ${(props) => props.theme.units(6)};

  display: flex;
  align-items: center;

  padding: 0 ${(props) => props.theme.units(2)};
`;

export const SongHeader = (props: SongHeaderProps) => {
  return (
    <Container selected={props.selected}>
      <Header>{props.name}</Header>
    </Container>
  );
};

const Header = styled.h2`
  ${LargeMain}
`;
