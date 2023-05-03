import React from 'react';
import styled from 'styled-components';
import {projectConstants} from '../../model/project';
import {AddChannel} from './AddChannel';
import {Channel} from './Channel';
import {useChannels} from './channels-hooks';

const Container = styled.div`
  display: flex;
  margin: ${(props) => props.theme.units(2)} ${(props) => props.theme.units(4)};
  overflow-x: scroll;
`;

export const Channels: React.FunctionComponent = () => {
  const channels = useChannels();

  return (
    <Container>
      {channels &&
        channels?.map((channel) => (
          <Channel key={channel.id} channelId={channel.id}></Channel>
        ))}
      {channels && channels.length < projectConstants.MAX_CHANNELS && (
        <AddChannel />
      )}
    </Container>
  );
};
