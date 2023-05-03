import React from 'react';
import styled from 'styled-components';
import {Section} from './Section';

interface SectionsProps {
  songId: string;
  sectionIds: string[];
}

const Container = styled.div`
  background: none;

  display: grid;
  grid-template-columns: 1fr;
  grid-gap: ${(props) => props.theme.units(2)};

  padding: ${(props) => props.theme.units(2)};

  @media only screen and (min-width: 600px) {
    grid-template-columns: repeat(2, 1fr);
  }
`;

export const Sections = (props: SectionsProps) => {
  return (
    <Container>
      {props.sectionIds.map((sectionId) => (
        <Section key={sectionId} songId={props.songId} sectionId={sectionId} />
      ))}
    </Container>
  );
};
