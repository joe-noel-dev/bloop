import React, {useRef, useState} from 'react';
import {FiMinus, FiPlus} from 'react-icons/fi';
import styled from 'styled-components';
import {MainTextStyle, MediumMain, MediumText} from '../typography/Typography';

interface Props {
  value: number;
  onSubmit(value: number): void;
  validate?(value: number): boolean;
  sanitise?(value: number): number;
  disabled?: boolean;
}

export const ValueEditor = (props: Props) => {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(`${props.value}`);
  const inputRef = useRef<HTMLInputElement>(null);

  const submit = (value: string) => {
    let numericValue = parseFloat(value);
    if (isNaN(numericValue)) {
      return;
    }

    if (props.validate && !props.validate(numericValue)) {
      return;
    }

    if (props.sanitise) {
      numericValue = props.sanitise(numericValue);
    }

    props.onSubmit(numericValue);
  };

  const onEditChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value);
  };

  const onClickOutside = () => {
    submit(value);
    setEditing(false);
  };

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      submit(value);
      setEditing(false);
    }

    if (event.key === 'Escape') {
      setValue(`${props.value}`);
      setEditing(false);
    }
  };

  return (
    <Container>
      <div style={{display: 'flex'}}>
        <IncrementButton
          onClick={(event) => {
            submit(`${props.value - 1}`);
            event.preventDefault();
          }}
        >
          <FiMinus size={16} />
        </IncrementButton>
        <ValueContainer
          onClick={() => {
            if (!editing && !props.disabled) setEditing(true);
          }}
        >
          {editing ? (
            <input
              autoFocus
              disabled={props.disabled}
              ref={inputRef}
              type="number"
              value={`${value}`}
              onKeyDown={onKeyDown}
              onChange={onEditChange}
              onBlur={onClickOutside}
            />
          ) : (
            <ValueText>{props.value}</ValueText>
          )}
        </ValueContainer>

        <IncrementButton
          onClick={(event) => {
            submit(`${props.value + 1}`);
            event.preventDefault();
          }}
        >
          <FiPlus size={16} />
        </IncrementButton>
      </div>
    </Container>
  );
};

const ValueText = styled.p`
  ${MediumMain};
`;

const Container = styled.div`
  width: ${(props) => props.theme.units(12)};
`;

const ValueContainer = styled.div`
  text-align: center;
  flex: 1;
  height: ${(props) => props.theme.units(4)};

  * {
    width: 100%;
    height: 100%;
    margin: 0 auto;
    line-height: ${(props) => props.theme.units(4)};
    text-align: center;
    ${MediumText};
    ${MainTextStyle};
  }

  input {
    border: 1px solid black;
    border-radius: ${(props) => props.theme.borderRadius};
  }
`;

const IncrementButton = styled.button`
  width: ${(props) => props.theme.units(4)};
  border: 1px solid black;
  border-radius: ${(props) => props.theme.borderRadius};
`;
