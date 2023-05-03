import React, {useState} from 'react';
import {FiChevronDown, FiRepeat, FiTrash} from 'react-icons/fi';
import styled from 'styled-components';
import {
  MainTextStyle,
  MediumMain,
  MediumText,
} from '../../typography/Typography';
import {
  NameEditor,
  NameEditorName,
  NameEditorTextInput,
} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import {cloneDeep} from 'lodash';
import {ValueEditor} from '../../components/ValueEditor';
import {LoopSelection} from './LoopSelection';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {appTheme} from '../theme';
import Measure from 'react-measure';
import {Waveform} from '../waveforms/Waveform';
import {beatLength} from '../../model/sample';
import {WarningButton} from '../../components/Button';
import {horizontalGap, verticalGap} from '../../components/Gap';
import {selectSectionRequest, updateSectionRequest} from '../../api/request';
import {useSampleWithId} from '../samples/sample-hooks';
import {useSectionById, useSelectedSectionId} from './section-hooks';

interface Props {
  sectionId: string;
  sampleId: string;
  editing: boolean;
  canRemove: boolean;
  onRequestEdit(edit: boolean): void;
  onRequestRemove(): void;
}

export const SectionEditor = (props: Props) => {
  const section = useSectionById(props.sectionId);
  const core = useCore();
  const selectedSection = useSelectedSectionId();
  const playbackState = usePlaybackState();
  const progress = useProgress();
  const [height, setHeight] = useState(0);
  const sample = useSampleWithId(props.sampleId);

  const length = sample ? beatLength(sample) : 0.0;

  const isPlaying =
    playbackState?.playing === 'playing' &&
    playbackState.sectionId === props.sectionId;

  const isSelected = selectedSection === props.sectionId;

  if (!section) {
    return <></>;
  }

  const submitSection = (changes: object) => {
    const newSection = cloneDeep(section);
    Object.assign(newSection, changes);
    core?.sendRequest(updateSectionRequest(newSection));
  };

  return (
    <Container
      selected={isSelected}
      onClick={() => {
        core?.sendRequest(selectSectionRequest(props.sectionId));
      }}
    >
      <Header
        onClick={() => {
          props.onRequestEdit(!props.editing);
        }}
      >
        <NameEditor
          onSave={(name) => {
            const newSection = cloneDeep(section);
            newSection.name = name;
            core?.sendRequest(updateSectionRequest(newSection));
          }}
          name={section.name}
          editable={props.editing}
        ></NameEditor>

        <Spacer />
        {section.loop && <FiRepeat size={16} />}

        <RevealButton
          editing={props.editing}
          onClick={(event) => {
            props.onRequestEdit(!props.editing);
            event.stopPropagation();
          }}
        >
          <FiChevronDown size={24} />
        </RevealButton>
      </Header>
      <SectionProperties
        style={{height: props.editing ? height : 0}}
        open={props.editing}
      >
        <Measure
          bounds
          onResize={(contentRect) => setHeight(contentRect.bounds?.height || 0)}
        >
          {({measureRef}) => (
            <InnerSectionProperties ref={measureRef}>
              <LoopPositionSection>
                <EditGroup>
                  <SectionHeading>Start at bar</SectionHeading>
                  <ValueEditor
                    value={section.start / 4.0 + 1.0}
                    onSubmit={(value) =>
                      submitSection({start: (value - 1) * 4.0})
                    }
                  />
                </EditGroup>
                <div
                  style={{flex: 1, alignSelf: 'stretch', position: 'relative'}}
                >
                  <Waveform
                    sampleId={props.sampleId}
                    start={length > 0.0 ? section.start / length : 0.0}
                    end={
                      length > 0.0
                        ? (section.start + section.beatLength) / length
                        : 1.0
                    }
                  />
                  {isPlaying && props.editing && (
                    <ProgressBar
                      progress={progress?.sectionProgress || 0}
                      colour={appTheme.colours.primaryDark}
                    />
                  )}
                </div>
                <EditGroup>
                  <SectionHeading>Duration</SectionHeading>
                  <ValueEditor
                    value={section.beatLength / 4.0}
                    onSubmit={(value) =>
                      submitSection({beatLength: value * 4.0})
                    }
                  />
                </EditGroup>
              </LoopPositionSection>

              <div style={{display: 'flex', alignItems: 'flex-end'}}>
                <EditGroup>
                  <SectionHeading>Loop</SectionHeading>
                  <LoopSelection
                    loop={section.loop}
                    onChange={(loop) => submitSection({loop})}
                  />
                </EditGroup>
                <Spacer />
                {props.editing && props.canRemove && (
                  <WarningButton
                    onClick={(event) => {
                      props.onRequestRemove();
                      event.stopPropagation();
                    }}
                  >
                    <FiTrash size={16} />
                    <ButtonText>Remove Section</ButtonText>
                  </WarningButton>
                )}
              </div>
            </InnerSectionProperties>
          )}
        </Measure>
      </SectionProperties>
      {isPlaying && !props.editing && (
        <ProgressBar
          progress={progress?.sectionProgress || 0}
          colour={appTheme.colours.primaryDark}
        />
      )}
    </Container>
  );
};

const SectionHeading = styled.h3`
  ${MediumMain};
`;

const ButtonText = styled.p`
  ${MediumMain}
`;

interface ContainerProps {
  selected: boolean;
}

const Container = styled.div<ContainerProps>`
  border: 1px solid
    ${(props) =>
      props.selected
        ? props.theme.textColours.card
        : props.theme.colours.cardLayer};
  border-radius: ${(props) => props.theme.borderRadius};
  padding: ${(props) => props.theme.units(2)};
  position: relative;
`;

const Header = styled.div`
  display: flex;
  align-items: center;

  height: ${(props) => props.theme.units(2)};

  ${NameEditorName} {
    ${MediumText};
    ${MainTextStyle};
  }

  ${NameEditorTextInput} {
    ${MediumText};
    ${MainTextStyle};
  }
`;

const Spacer = styled.div`
  flex: 1;
`;

interface ButtonProps {
  editing: boolean;
}

const RevealButton = styled.button<ButtonProps>`
  transform: ${(props) => (props.editing ? 'rotate(-180deg)' : 'rotate(0)')};
  transition: transform 0.2s ease-in-out;
`;

interface SectionPropertiesProps {
  open: boolean;
}

const LoopPositionSection = styled.div`
  display: flex;
  ${(props) => horizontalGap(props.theme.units(2))};

  align-items: center;
`;

const SectionProperties = styled.div<SectionPropertiesProps>`
  transition: all 0.2s ease-out;
  overflow: hidden;
  margin-top: ${(props) => (props.open ? props.theme.units(2) : '0')};
`;

const EditGroup = styled.div`
  display: flex;
  flex-direction: column;
  ${(props) => verticalGap(props.theme.units(1))};
`;

const InnerSectionProperties = styled.div`
  display: flex;
  flex-direction: column;
  ${(props) => verticalGap(props.theme.units(2))};
`;
