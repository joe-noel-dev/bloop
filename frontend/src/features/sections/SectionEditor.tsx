import {useState} from 'react';
import {FiChevronDown, FiRepeat, FiTrash} from 'react-icons/fi';
import {NameEditor} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import cloneDeep from 'lodash.clonedeep';
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
import {ToggleSwitch} from '../../components/ToggleSwitch';
import {NumberChooser} from '../../components/NumberChooser';
import {Section} from '../../model/section';

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

  return (
    <div
      className={`${styles.container} ${
        isSelected && styles['container-selected']
      }`}
      onClick={() => {
        core?.sendRequest(selectSectionRequest(props.sectionId));
      }}
    >
      <Header
        editing={props.editing}
        selected={isSelected}
        section={section}
        onRequestEdit={props.onRequestEdit}
      />

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

              <Properties section={section} />

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

interface HeaderProps {
  editing: boolean;
  selected: boolean;
  section: Section;
  onRequestEdit(edit: boolean): void;
}

const Header = ({editing, selected, section, onRequestEdit}: HeaderProps) => {
  const core = useCore();

  return (
    <div
      className={styles.header}
      onClick={() => {
        if (selected && !editing) {
          onRequestEdit(true);
        }
      }}
    >
      <NameEditor
        onSave={(name) => {
          const newSection = cloneDeep(section);
          newSection.name = name;
          core?.sendRequest(updateSectionRequest(newSection));
        }}
        name={section.name}
        editable={editing}
      />

      <Spacer />

      {section.loop && <FiRepeat />}

      <button
        className={`${styles['reveal-button']} ${
          editing && styles['reveal-button-editing']
        }`}
        onClick={(event) => {
          onRequestEdit(!editing);
          event.stopPropagation();
        }}
      >
        <FiChevronDown size={24} />
      </button>
    </div>
  );
};

interface PropertiesProps {
  section: Section;
}

const Properties = ({section}: PropertiesProps) => {
  const core = useCore();

  const submitSection = (changes: object) => {
    const newSection = cloneDeep(section);
    Object.assign(newSection, changes);
    core?.sendRequest(updateSectionRequest(newSection));
  };

  return (
    <>
      <div className={styles['edit-group']}>
        <h3>Start</h3>
        <NumberChooser
          value={section.start}
          onValueChange={(value) => submitSection({start: value})}
        />
      </div>

      <div className={styles.separator} />

      <div className={styles['edit-group']}>
        <h3>Duration</h3>
        <NumberChooser
          value={section.beatLength}
          onValueChange={(value) => submitSection({beatLength: value})}
        />
      </div>

      <div className={styles.separator} />

      <div className={styles['edit-group']}>
        <h3>Loop</h3>
        <ToggleSwitch
          isOn={section.loop}
          onChange={(loop) => submitSection({loop})}
        />
      </div>
    </>
  );
};
