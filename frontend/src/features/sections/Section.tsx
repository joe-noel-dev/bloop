import {FiRepeat} from 'react-icons/fi';
import {selectSectionRequest} from '../../api/request';
import {ProgressBar} from '../../components/ProgressBar';
import {Spacer} from '../../components/Spacer';
import {useCore} from '../core/use-core';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSelectedSectionId} from './section-hooks';
import styles from './Section.module.css';
import {Section as ModelSection} from '../../model/section';

interface SectionProps {
  section: ModelSection;
}

export const Section = ({section}: SectionProps) => {
  const selectedSectionId = useSelectedSectionId();

  const core = useCore();
  const isSelected = section?.id === selectedSectionId;

  const playbackState = usePlaybackState();
  const progress = useProgress();

  const isPlaying =
    playbackState?.playing === 'playing' &&
    playbackState.sectionId === section.id;

  const isCued =
    playbackState?.playing === 'playing' &&
    playbackState.queuedSectionId === section.id;

  return (
    <div
      className={`${styles['container']} ${
        isSelected && styles['container-selected']
      } ${isPlaying && styles['container-playing']} ${
        isCued && styles['container-cued']
      }`}
      onClick={(event) => {
        if (core && section) {
          core.sendRequest(selectSectionRequest(section.id));
        }

        event.stopPropagation();
      }}
    >
      <h3>{section?.name}</h3>
      <Spacer />
      {section?.loop && <FiRepeat size={16} />}
      {isPlaying && (
        <ProgressBar
          progress={progress?.sectionProgress || 0.0}
          colour={'var(--primary-dark)'}
        />
      )}
    </div>
  );
};
