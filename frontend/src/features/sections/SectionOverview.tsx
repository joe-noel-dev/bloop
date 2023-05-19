import {selectSectionRequest} from '../../api/request';
import {ProgressBar} from '../../components/ProgressBar';
import {getSampleBeatLength} from '../../model/sample';
import {Section} from '../../model/section';
import {Song, getSectionBeatLength} from '../../model/song';
import {useCore} from '../core/use-core';
import {useSelectedSongId} from '../songs/song-hooks';
import {usePlaybackState, useProgress} from '../transport/transport-hooks';
import styles from './SectionOverview.module.css';
import {useSelectedSectionId} from './section-hooks';

interface Props {
  song: Song;
}

export const SectionOverview = ({song}: Props) => {
  const selectedSectionId = useSelectedSectionId();
  const selectedSongId = useSelectedSongId();
  const playbackState = usePlaybackState();
  const core = useCore();
  const progress = useProgress();

  const sample = song.sample;
  if (!sample) {
    return <></>;
  }

  const sampleBeatLength = getSampleBeatLength(sample);
  if (sampleBeatLength <= 0.0) {
    return <></>;
  }

  const normalisedSectionStart = (section: Section) =>
    section.start / sampleBeatLength;

  const normalisedSectionDuration = (section: Section) =>
    getSectionBeatLength(song, section.id) / sampleBeatLength;

  const isSectionSelected = (section: Section) =>
    song.id === selectedSongId && section.id === selectedSectionId;

  const isSectionPlaying = (section: Section) =>
    playbackState?.playing === 'playing' &&
    playbackState.sectionId === section.id;

  const isSectionQueued = (section: Section) =>
    playbackState?.playing && playbackState.queuedSectionId === section.id;

  const selectSection = (section: Section) =>
    core.sendRequest(selectSectionRequest(section.id));

  return (
    <div className={styles.container}>
      {song.sections.map((section) => {
        const start = normalisedSectionStart(section);
        const duration = normalisedSectionDuration(section);

        const sectionStyles = [
          styles.section,
          isSectionSelected(section) && styles['section-selected'],
          isSectionPlaying(section) && styles['section-playing'],
          isSectionQueued(section) && styles['section-queued'],
        ];

        return (
          <div
            style={{position: 'relative'}}
            key={section.id}
            onClick={(event) => {
              selectSection(section);
              event.stopPropagation();
            }}
          >
            <div
              className={sectionStyles.join(' ')}
              style={{
                transform: `translateX(${start * 100}%)`,
              }}
            >
              <label>{section.name}</label>
            </div>
            <div
              className={styles['progress-bar-container']}
              style={
                {
                  '--start': `${start * 100}%`,
                  '--duration': `${duration * 100}%`,
                } as React.CSSProperties
              }
            >
              {isSectionPlaying(section) && progress && (
                <ProgressBar
                  colour="var(--primary)"
                  progress={progress.sectionProgress}
                />
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
};
