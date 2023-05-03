import React, {useRef, useState} from 'react';
import {FiCheck, FiX} from 'react-icons/fi';
import styled from 'styled-components';
import {SecondaryButton, SecondaryDarkButton} from '../../components/Button';
import styles from './RenameProject.module.css';

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
    <div className={styles['container']}>
      <Title>
        <h2>{props.title || 'Project Name'}</h2>
      </Title>

      <input
        className={styles['name-box']}
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
          <p>Cancel</p>
        </SecondaryDarkButton>
        <Spacer />
        <SecondaryButton onClick={() => props.onSave(value)}>
          <FiCheck size={16} />
          <p>{props.confirmButtonText || 'Create'}</p>
        </SecondaryButton>
      </ButtonContainer>
    </div>
  );
};

const NameBox = styled.input``;

const ButtonContainer = styled.div`
  display: flex;
`;

const Spacer = styled.div`
  flex: 1;
`;

const Title = styled.div`
  margin-bottom: ${(props) => props.theme.units(2)};
`;
