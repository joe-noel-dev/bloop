import React from 'react';
import {FiRepeat} from 'react-icons/fi';
import styled from 'styled-components';
import {selectSectionRequest} from '../../api/request';
import {ProgressBar} from '../../components/ProgressBar';
import {Spacer} from '../../components/Spacer';
import {MediumMain} from '../../typography/Typography';
import {useCore} from '../core/use-core';
import {appTheme} from '../theme';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSectionById, useSelectedSectionId} from './section-hooks';

interface SectionProps {
  songId: string;
  sectionId: string;
}

export const Section = (props: SectionProps) => {
  const section = useSectionById(props.sectionId);
  const selectedSectionId = useSelectedSectionId();

  const core = useCore();
  const isSelected = section?.id === selectedSectionId;

  const playbackState = usePlaybackState();
  const progress = useProgress();

  const isPlaying =
    playbackState?.playing === 'playing' &&
    playbackState.sectionId === props.sectionId;

  return (
    <Container
      selected={isSelected}
      playing={isPlaying}
      onClick={(event) => {
        if (core && section) {
          core.sendRequest(selectSectionRequest(section.id));
        }

        event.stopPropagation();
      }}
    >
      <SectionTitle>{section?.name}</SectionTitle>
      <Spacer />
      {section?.loop && <FiRepeat size={16} />}
      {isPlaying && (
        <ProgressBar
          progress={progress?.sectionProgress || 0.0}
          colour={appTheme.colours.primaryDark}
        />
      )}
    </Container>
  );
};

const SectionTitle = styled.h3`
  ${MediumMain}
`;

interface ContainerProps {
  selected: boolean;
  playing: boolean;
}

const Container = styled.div<ContainerProps>`
  padding: 0 ${(props) => props.theme.units(2)};
  height: ${(props) => props.theme.units(6)};

  align-items: center;

  position: relative;

  background: ${(props) =>
    props.playing
      ? props.theme.colours.primaryLight
      : props.theme.colours.cardBackground};
  color: ${(props) =>
    props.playing
      ? props.theme.textColours.primaryLight
      : props.theme.textColours.card};

  border-radius: ${(props) => props.theme.borderRadius};
  border: 1px solid
    ${(props) =>
      props.selected
        ? props.theme.textColours.card
        : props.theme.colours.cardLayer};

  display: flex;
`;
