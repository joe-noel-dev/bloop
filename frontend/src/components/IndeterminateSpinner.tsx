import React from 'react';
import styled, {css, keyframes} from 'styled-components';
import {appTheme} from '../features/theme';

export const IndeterminateSpinner = () => {
  return (
    <Spinner>
      <LeftWrapper>
        <Left>
          <Circle />
        </Left>
      </LeftWrapper>
      <RightWrapper>
        <Right>
          <Circle />
        </Right>
      </RightWrapper>
    </Spinner>
  );
};

const radius = appTheme.units(4);
const border = appTheme.units(0.5);

const spin = keyframes`
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(2520deg);
  }
`;

const Spinner = styled.div`
  position: relative;

  width: calc(2 * ${radius});
  height: calc(2 * ${radius});

  animation: ${spin} 10s linear infinite;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    border: ${border} solid #d4d7dc;
    border-radius: ${radius};
  }
`;

const Circle = styled.div`
  position: absolute;
  border: ${border} solid ${(props) => props.theme.colours.primaryDark};
  width: calc(2 * ${radius});
  height: calc(2 * ${radius});
  border-radius: ${radius};
`;

const spinLeft = keyframes`
  0% {
    transform: rotate(20deg);
  }
  50% {
    transform: rotate(160deg);
  }
  100% {
    transform: rotate(20deg);
  }
`;

const spinRight = keyframes`
  0% {
    transform: rotate(160deg);
  }
  50% {
    transform: rotate(20deg);
  }
  100% {
    transform: rotate(160deg);
  }
`;

const Half = css`
  position: absolute;
  top: 0;
  overflow: hidden;
  width: ${radius};
  height: calc(2 * ${radius});
`;

const LeftHalf = css`
  ${Half};
  left: 0;
`;

const RightHalf = css`
  ${Half};
`;

const Left = styled.div`
  ${LeftHalf};
  transform-origin: 100% 50%;
  animation: ${spinLeft} 2.5s cubic-bezier(0.2, 0, 0.8, 1) infinite;
`;

const Right = styled.div`
  ${RightHalf};
  transform-origin: 100% 50%;
  animation: ${spinRight} 2.5s cubic-bezier(0.2, 0, 0.8, 1) infinite;
  left: calc(-${radius});
`;

const LeftWrapper = styled.div`
  ${LeftHalf};
`;

const RightWrapper = styled.div`
  ${RightHalf};
  right: 0;
`;
