import React from 'react';
import styled from 'styled-components';
import {useCore} from '../core/use-core';
import {FaPlus} from 'react-icons/fa';
import styles from './styles';
import {addChannelRequest} from '../../api/request';

const Button = styled.div`
  margin: ${styles.margin};

  width: ${styles.width};
  height: ${styles.height};
  border-radius: ${(props) => props.theme.borderRadius};
  background-color: ${(props) => props.theme.colours.cardLayer};

  color: white;
  font-size: 1.5rem;

  display: flex;
  justify-content: center;
  align-items: center;

  opacity: 50%;
`;

export const AddChannel = () => {
  const core = useCore();

  const addChannel = () => {
    core?.sendRequest(addChannelRequest());
  };
  return (
    <Button onClick={addChannel}>
      <FaPlus size={'1rem'} color={'black'} />
    </Button>
  );
};
