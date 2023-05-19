import {useState} from 'react';
import {FiChevronDown, FiRepeat, FiTrash} from 'react-icons/fi';
import {NameEditor} from '../../components/NameEditor';
import {useCore} from '../core/use-core';
import cloneDeep from 'lodash.clonedeep';
import {ProgressBar} from '../../components/ProgressBar';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import Measure from 'react-measure';
import {Waveform} from '../waveforms/Waveform';
import {Sample, getSampleBeatLength} from '../../model/sample';
import {WarningButton} from '../../components/Button';
import {selectSectionRequest, updateSectionRequest} from '../../api/request';
import {useSelectedSectionId} from './section-hooks';
import styles from './SectionEditor.module.css';
import {Spacer} from '../../components/Spacer';
import {ToggleSwitch} from '../../components/ToggleSwitch';
import {NumberChooser} from '../../components/NumberChooser';
import {Section} from '../../model/section';
import {Song, getSectionBeatLength} from '../../model/song';

interface Props {
  section: Section;
  song: Song;
  sample?: Sample;
  editing: boolean;
  canRemove: boolean;
  onRequestEdit(edit: boolean): void;
  onRequestRemove(): void;
}

export const SectionEditor = ({
  section,
  song,
  sample,
  editing,
  canRemove,
  onRequestEdit,
  onRequestRemove,
}: Props) => {
  const core = useCore();
  const selectedSection = useSelectedSectionId();
  const playbackState = usePlaybackState();
  const progress = useProgress();
  const [height, setHeight] = useState(0);
  const sectionLength = getSectionBeatLength(song, section.id);

  const sampleLength = sample ? getSampleBeatLength(sample) : 0.0;

  const isPlaying =
    playbackState?.playing === 'playing' &&
    playbackState.sectionId === section.id;

  const isSelected = selectedSection === section.id;

  const isQueued =
    playbackState?.playing && playbackState.queuedSectionId === section.id;

  if (!section) {
    return <></>;
  }

  const containerStyles = [
    styles.container,
    isSelected && styles['container-selected'],
    isPlaying && styles['container-playing'],
    isQueued && styles['container-queued'],
  ];

  return (
    <div
      className={containerStyles.join(' ')}
      onClick={(event) => {
        core?.sendRequest(selectSectionRequest(section.id));
        event.stopPropagation();
      }}
    >
      <Header
        editing={editing}
        selected={isSelected}
        section={section}
        onRequestEdit={onRequestEdit}
      />

      <div
        className={`${styles['section-properties']} ${
          editing && styles['section-properties-open']
        }`}
        style={{height: editing ? height : 0}}
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
                  sample={sample}
                  start={
                    sampleLength > 0.0 ? section.start / sampleLength : 0.0
                  }
                  end={
                    sampleLength > 0.0
                      ? (section.start + sectionLength) / sampleLength
                      : 1.0
                  }
                />
                {isPlaying && editing && (
                  <ProgressBar
                    progress={progress?.sectionProgress || 0}
                    colour={'var(--primary-dark)'}
                  />
                )}
              </div>

              <Properties section={section} />

              {editing && canRemove && (
                <WarningButton
                  onClick={(event) => {
                    onRequestRemove();
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
      {isPlaying && !editing && (
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
        <h3>Loop</h3>
        <ToggleSwitch
          isOn={section.loop}
          onChange={(loop) => submitSection({loop})}
        />
      </div>

      <div className={styles['edit-group']}>
        <h3>Metronome</h3>
        <ToggleSwitch
          isOn={section.metronome}
          onChange={(metronome) => submitSection({metronome})}
        />
      </div>
    </>
  );
};
