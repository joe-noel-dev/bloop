import React from 'react';
import styled from 'styled-components';

interface Props {
  progress: number;
  colour: string;
}

export const ProgressBar = ({progress, colour}: Props) => {
  return (
    <Container
      style={
        {
          '--progress': `${progress * 100.0}%`,
          '--colour': colour,
        } as React.CSSProperties
      }
    />
  );
};

const Container = styled.div`
  position: absolute;
  bottom: 0;
  height: 5px;
  left: 0;
  background: var(--colour);
  width: var(--progress);
  transition: width 0.05;
`;
