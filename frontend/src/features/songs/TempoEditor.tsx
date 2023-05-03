import React, {useEffect, useRef, useState} from 'react';
import {Popover} from 'react-tiny-popover';
import styled from 'styled-components';

interface TempoEditorProps {
  value: number;
  onValueChanged(newValue: number): void;
}

const Label = styled.p`
  margin: 0;
  border: 1px solid ${(props) => props.theme.textColours.card};
  border-radius: ${(props) => props.theme.borderRadius};
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 0.7rem;
`;

const Container = styled.div`
  background-color: ${(props) => props.theme.colours.cardBackground};
  color: ${(props) => props.theme.textColours.card};
  padding: 0.5rem;

  display: flex;
  flex-direction: column;
  align-items: center;

  border: 1px solid ${(props) => props.theme.textColours.card};
  border-radius: ${(props) => props.theme.borderRadius};
  transition: ${(props) => props.theme.transition};
`;

const Title = styled.h3`
  color: inherit;
  margin: 0.5rem;
  text-align: center;
`;

const Button = styled.div`
  width: 5rem;
  padding: 0.5rem;
  text-align: center;

  margin: 0.5rem;
`;

const CancelButton = styled(Button)`
  background-color: ${(props) => props.theme.colours.primaryLight};
`;

const SaveButton = styled(Button)`
  background-color: ${(props) => props.theme.colours.secondary};
`;

const Buttons = styled.div`
  display: flex;
`;

const NumberInput = styled.input`
  margin: 0.5rem;
  outline: none;
  border: 1px solid ${(props) => props.theme.textColours.card};
  border-radius: 2px;
  font-size: 1.5rem;
  padding: 0.5rem;
  width: 8rem;
  text-align: center;
`;

interface EditPanelProps {
  onSave(value: number): void;
  onCancel(): void;
  initialValue: number;
}

const EditPanel = ({initialValue, onSave, onCancel}: EditPanelProps) => {
  const [value, setValue] = useState(initialValue);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => inputRef.current?.select(), [inputRef]);

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      onSave(value);
    }

    if (e.key === 'Escape') {
      onCancel();
    }
  };

  return (
    <Container>
      <Title>Tempo</Title>
      <NumberInput
        type="number"
        value={value}
        ref={inputRef}
        onChange={(e) => setValue(parseFloat(e.target.value))}
        onKeyDown={handleKeyPress}
      />
      <Buttons>
        <CancelButton onClick={onCancel}>Cancel</CancelButton>
        <SaveButton onClick={() => onSave(value)}>Save</SaveButton>
      </Buttons>
    </Container>
  );
};

export const TempoEditor = ({value, onValueChanged}: TempoEditorProps) => {
  const [editingTempo, setEditingTempo] = useState(false);

  const renderTempoEditor = () => (
    <EditPanel
      onSave={(value) => {
        onValueChanged(value);
        setEditingTempo(false);
      }}
      onCancel={() => setEditingTempo(false)}
      initialValue={value}
    />
  );

  return (
    <Popover
      positions={['bottom', 'right', 'left', 'top']}
      content={renderTempoEditor}
      isOpen={editingTempo}
      onClickOutside={() => setEditingTempo(false)}
    >
      <Label onClick={() => setEditingTempo(true)}>{`${value} bpm`}</Label>
    </Popover>
  );
};
