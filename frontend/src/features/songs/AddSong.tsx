import React from 'react';
import {FiPlus} from 'react-icons/fi';
import styled from 'styled-components';
import {addSongRequest} from '../../api/request';
import {SecondaryButton} from '../../components/Button';
import {MediumMain} from '../../typography/Typography';
import {useCore} from '../core/use-core';

export const AddSong = () => {
  const core = useCore();

  const addSong = () => {
    core?.sendRequest(addSongRequest());
  };

  return (
    <SecondaryButton onClick={addSong}>
      <FiPlus size={16} />
      <ButtonText>Add Song</ButtonText>
    </SecondaryButton>
  );
};

const ButtonText = styled.p`
  ${MediumMain};
`;
