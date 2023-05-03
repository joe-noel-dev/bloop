import React from 'react';
import styled from 'styled-components';
import {useChannel} from './channels-hooks';
import styles from './styles';

interface ChannelProps {
  channelId: string;
}

interface ContainerProps {
  backgroundColour: string;
}

const Container = styled.div<ContainerProps>`
  width: ${styles.width};
  height: ${styles.height};
  box-shadow: ${(props) => props.theme.dropShadow};
  background: ${(props) => props.backgroundColour};
  border-radius: ${styles.borderRadius};

  margin: ${styles.margin};

  display: flex;
  align-items: center;
  justify-content: center;

  flex-shrink: 0;

  animation: 0.5s ${(props) => props.theme.fadeInKeyFrames} ease-in-out;
`;

const ChannelName = styled.h2`
  font-size: 0.8em;
  text-align: center;
  color: 'black';
`;

export const Channel = (props: ChannelProps) => {
  const channel = useChannel(props.channelId);

  return (
    <Container backgroundColour={channel?.colour || 'white'}>
      <ChannelName>{channel?.name}</ChannelName>
    </Container>
  );
};
