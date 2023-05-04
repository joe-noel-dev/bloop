import {FiRepeat} from 'react-icons/fi';
import {selectSectionRequest} from '../../api/request';
import {ProgressBar} from '../../components/ProgressBar';
import {Spacer} from '../../components/Spacer';
import {useCore} from '../core/use-core';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import {useSectionById, useSelectedSectionId} from './section-hooks';
import styles from './Section.module.css';

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
    <div
      className={`${styles['container']} ${
        isSelected && styles['container-selected']
      } ${isPlaying && styles['container-playing']}`}
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
