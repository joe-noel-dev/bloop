import React from 'react';
import styled from 'styled-components';
import {verticalGap} from '../../components/Gap';
import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import {LargeMain} from '../../typography/Typography';

export const NoConnectionOverlay = () => {
  return (
    <Container>
      <Text>Waiting for connection...</Text>
      <IndeterminateSpinner />
    </Container>
  );
};

const Text = styled.h3`
  ${LargeMain}
`;

const Container = styled.div`
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  ${(props) => verticalGap(props.theme.units(2))};
`;
