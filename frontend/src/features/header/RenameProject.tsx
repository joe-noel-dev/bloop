import React, {useRef, useState} from 'react';
import {FiCheck, FiX} from 'react-icons/fi';
import styled from 'styled-components';
import {SecondaryButton, SecondaryDarkButton} from '../../components/Button';
import {
  LargeMain,
  MainTextStyle,
  MediumMain,
  MediumText,
} from '../../typography/Typography';

interface Props {
  name?: string;
  onSave(name: string): void;
  onCancel(): void;
  confirmButtonText?: string;
  title?: string;
}

export const RenameProject = (props: Props) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(props.name || '');

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      if (inputRef.current && inputRef.current.value) {
        props.onSave(inputRef.current.value);
      }
    }

    if (event.key === 'Escape') {
      props.onCancel();
    }
  };

  return (
    <Container>
      <Title>
        <ProjectNameText>{props.title || 'Project Name'}</ProjectNameText>
      </Title>

      <NameBox
        autoFocus
        onKeyDown={onKeyDown}
        ref={inputRef}
        type="text"
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />
      <ButtonContainer>
        <SecondaryDarkButton onClick={props.onCancel}>
          <FiX size={16} />
          <ButtonText>Cancel</ButtonText>
        </SecondaryDarkButton>
        <Spacer />
        <SecondaryButton onClick={() => props.onSave(value)}>
          <FiCheck size={16} />
          <ButtonText>{props.confirmButtonText || 'Create'}</ButtonText>
        </SecondaryButton>
      </ButtonContainer>
    </Container>
  );
};

const ProjectNameText = styled.h2`
  ${LargeMain}
`;

const ButtonText = styled.p`
  ${MediumMain}
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  width: ${(props) => props.theme.units(50)};
  padding: ${(props) => props.theme.units(4)};
`;

const NameBox = styled.input`
  height: ${(props) => props.theme.units(6)};
  border-radius: ${(props) => props.theme.borderRadius};
  border: 1px solid ${(props) => props.theme.colours.cardLayer};
  padding: 0 ${(props) => props.theme.units(1)};
  margin-bottom: ${(props) => props.theme.units(2)};
  ${MediumText}
  ${MainTextStyle}
`;

const ButtonContainer = styled.div`
  display: flex;
`;

const Spacer = styled.div`
  flex: 1;
`;

const Title = styled.div`
  margin-bottom: ${(props) => props.theme.units(2)};
`;
