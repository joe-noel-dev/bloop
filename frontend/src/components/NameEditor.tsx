import React, {useRef, useState} from 'react';
import {FiEdit2} from 'react-icons/fi';
import {LargeMain, LargeText, MainTextStyle} from '../typography/Typography';
import styled from 'styled-components';

interface NameEditorProps {
  onSave(name: string): void;
  name: string;
  editable: boolean;
  inputType?: string;
}

export const NameEditor = (props: NameEditorProps) => {
  const [editing, setEditing] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(props.name);

  const startEditing = () => {
    if (!editing && props.editable) {
      setEditing(true);
      inputRef.current?.focus();
    }
  };

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (!props.editable) return;

    if (event.key === 'Enter') {
      if (inputRef.current && inputRef.current.value) {
        props.onSave(inputRef.current.value);
      }

      setEditing(false);
    }

    if (event.key === 'Escape') {
      setValue(props.name);
      setEditing(false);
    }
  };

  const onClickOutside = () => {
    if (!props.editable) return;

    if (inputRef.current && inputRef.current.value) {
      props.onSave(inputRef.current.value);
    }

    if (editing) {
      setEditing(false);
    }
  };

  return (
    <Container
      onClick={(event) => {
        startEditing();
        event.stopPropagation();
      }}
    >
      {!editing && (
        <NameEditorName>
          {props.name}
          {props.editable && (
            <EditButton>
              <FiEdit2 size={16} />
            </EditButton>
          )}
        </NameEditorName>
      )}
      {editing && (
        <NameEditorTextInput
          autoFocus
          onKeyDown={onKeyDown}
          onBlur={onClickOutside}
          ref={inputRef}
          type={props.inputType || 'text'}
          value={value}
          onChange={(event) => setValue(event.target.value)}
        />
      )}
    </Container>
  );
};

const Container = styled.div``;

export const NameEditorName = styled.h3`
  ${LargeMain};
`;

export const NameEditorTextInput = styled.input`
  border: none;
  ${LargeText}
  ${MainTextStyle}
`;

const EditButton = styled.button`
  background: none;
  border: none;
  padding: 0 ${(props) => props.theme.units(1)};
`;
