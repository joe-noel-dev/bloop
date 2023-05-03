import React from 'react';
import styled from 'styled-components';
import {MediumMain} from '../../typography/Typography';

interface Props {
  loop: boolean;
  onChange(loop: boolean): void;
}

export const LoopSelection = (props: Props) => {
  interface SegmentOptions {
    name: string;
    action: () => void;
    selected: boolean;
  }

  const segment = (options: SegmentOptions) => {
    return (
      <RadioSegment
        onClick={(event) => {
          options.action();
          event.stopPropagation();
        }}
        selected={options.selected}
      >
        <SegmentName>{options.name}</SegmentName>
      </RadioSegment>
    );
  };

  return (
    <Container>
      <RadioBox>
        {segment({
          name: 'Off',
          action: () => props.onChange(false),
          selected: !props.loop,
        })}

        {segment({
          name: 'On',
          action: () => props.onChange(true),
          selected: props.loop,
        })}
      </RadioBox>
    </Container>
  );
};

const SegmentName = styled.p`
  ${MediumMain};
`;

const Container = styled.div`
  display: flex;
  height: ${(props) => props.theme.units(4)};
  line-height: ${(props) => props.theme.units(4)};
`;

const RadioBox = styled.div`
  display: flex;
  border-radius: ${(props) => props.theme.borderRadius};
  border: 1px solid ${(props) => props.theme.textColours.card};
`;

interface SegmentProps {
  selected: boolean;
}

const RadioSegment = styled.div<SegmentProps>`
  text-align: center;
  width: ${(props) => props.theme.units(6)};
  background: ${(props) =>
    props.selected ? props.theme.colours.primary : 'none'};
  color: ${(props) =>
    props.selected
      ? props.theme.textColours.primary
      : props.theme.textColours.card};
`;
