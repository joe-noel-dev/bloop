import React, {useState} from 'react';
import {FiChevronDown, FiRepeat, FiTrash} from 'react-icons/fi';
import {NameEditor} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import {cloneDeep} from 'lodash';
import {ValueEditor} from '../../components/ValueEditor';
import {LoopSelection} from './LoopSelection';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import Measure from 'react-measure';
import {Waveform} from '../waveforms/Waveform';
import {beatLength} from '../../model/sample';
import {WarningButton} from '../../components/Button';
import {selectSectionRequest, updateSectionRequest} from '../../api/request';
import {useSampleWithId} from '../samples/sample-hooks';
import {useSectionById, useSelectedSectionId} from './section-hooks';
import styles from './SectionEditor.module.css';
import {Spacer} from '../../components/Spacer';

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
    <div
      className={`${styles.container} ${
        isSelected && styles['container-selected']
      }`}
      onClick={() => {
        core?.sendRequest(selectSectionRequest(props.sectionId));
      }}
    >
      <div
        className={styles.header}
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

        <button
          className={`${styles['reveal-button']} ${
            props.editing && styles['reveal-button-editing']
          }`}
          onClick={(event) => {
            props.onRequestEdit(!props.editing);
            event.stopPropagation();
          }}
        >
          <FiChevronDown size={24} />
        </button>
      </div>
      <div
        className={`${styles['section-properties']} ${
          props.editing && styles['section-properties-open']
        }`}
        style={{height: props.editing ? height : 0}}
      >
        <Measure
          bounds
          onResize={(contentRect) => setHeight(contentRect.bounds?.height || 0)}
        >
          {({measureRef}) => (
            <div
              className={styles['inner-section-properties']}
              ref={measureRef}
            >
              <div className={styles['loop-position-section']}>
                <div className={styles['edit-group']}>
                  <h3>Start at bar</h3>
                  <ValueEditor
                    value={section.start / 4.0 + 1.0}
                    onSubmit={(value) =>
                      submitSection({start: (value - 1) * 4.0})
                    }
                  />
                </div>

                <div className={styles.waveform}>
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
                      colour={'var(--primary-dark)'}
                    />
                  )}
                </div>

                <div className={styles['edit-group']}>
                  <h3>Duration</h3>
                  <ValueEditor
                    value={section.beatLength / 4.0}
                    onSubmit={(value) =>
                      submitSection({beatLength: value * 4.0})
                    }
                  />
                </div>
              </div>

              <div style={{display: 'flex', alignItems: 'flex-end'}}>
                <div className={styles['edit-group']}>
                  <h3>Loop</h3>
                  <LoopSelection
                    loop={section.loop}
                    onChange={(loop) => submitSection({loop})}
                  />
                </div>
                <Spacer />
                {props.editing && props.canRemove && (
                  <WarningButton
                    onClick={(event) => {
                      props.onRequestRemove();
                      event.stopPropagation();
                    }}
                  >
                    <FiTrash size={16} />
                    <label>Remove Section</label>
                  </WarningButton>
                )}
              </div>
            </div>
          )}
        </Measure>
      </div>
      {isPlaying && !props.editing && (
        <ProgressBar
          progress={progress?.sectionProgress || 0}
          colour={'var(--primary-dark)'}
        />
      )}
    </div>
  );
};
