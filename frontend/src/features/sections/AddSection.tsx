import React from 'react';
import styled from 'styled-components';
import {useCore} from '../core/use-core';
import {FaPlus} from 'react-icons/fa';
import {addSectionRequest} from '../../api/request';

const Button = styled.div`
  background: ${(props) => props.theme.colours.cardLayer};
  color: ${(props) => props.theme.textColours.card};

  border-radius: ${(props) => props.theme.borderRadius};
  box-shadow: ${(props) => props.theme.dropShadow};
  border: none;
  padding: ${(props) => props.theme.units(2)};

  display: flex;
  justify-content: center;
  align-items: center;

  opacity: 75%;
`;

interface AddSectionProps {
  songId: string;
}

export const AddSection = (props: AddSectionProps) => {
  const core = useCore();

  const addSection = () => core?.sendRequest(addSectionRequest(props.songId));

  return (
    <Button onClick={addSection}>
      <FaPlus size={'1rem'} color={'black'} />
    </Button>
  );
};
