import {useCore} from '../core/use-core';
import nextSectionIcon from './next-section-icon.svg';
import nextSongIcon from './next-song-icon.svg';
import playIcon from './play-icon.svg';
import stopIcon from './stop-icon.svg';
import prevSectionIcon from './prev-section-icon.svg';
import prevSongIcon from './prev-song-icon.svg';
import {FiCheck, FiCornerUpRight, FiRepeat} from 'react-icons/fi';
import {useHotkeys} from 'react-hotkeys-hook';
import {
  useSelectedSong,
  useSelectedSongId,
  useSongs,
} from '../songs/song-hooks';
import {
  exitLoopRequest,
  loopRequest,
  playRequest,
  queueRequest,
  selectSectionRequest,
  selectSongRequest,
  stopRequest,
} from '../../api/request';
import {useSelectedSectionId} from '../sections/section-hooks';
import {Song} from '../../model/song';
import {usePlaybackState} from './transport-hooks';
import styles from './Transport.module.css';

export const Transport: React.FunctionComponent = () => {
  const playbackState = usePlaybackState();
  const core = useCore();

  const playing = playbackState?.playing === 'playing';
  const looping = playbackState?.playing === 'playing' && playbackState.looping;

  const selectedSectionId = useSelectedSectionId();
  const selectedSongId = useSelectedSongId();
  const selectedSong = useSelectedSong();
  const songs = useSongs();

  const selectedSectionIsPlaying =
    playing && selectedSectionId === playbackState.sectionId;

  const queueSection = () => {
    if (!selectedSongId || !selectedSectionId || !core) {
      return;
    }

    if (playbackState?.queuedSectionId === selectedSectionId) {
      return;
    }

    core.sendRequest(queueRequest(selectedSongId, selectedSectionId));
  };

  const incrementSongSelection = (delta: number) => {
    if (!songs) {
      return;
    }

    const index = songs.findIndex((song: Song) => song.id === selectedSongId);
    if (index === -1) {
      return;
    }

    const newIndex = index + delta;
    if (0 <= newIndex && newIndex < songs.length) {
      core?.sendRequest(selectSongRequest(songs[newIndex].id));
    }
  };

  const incrementSectionSelection = (delta: number) => {
    if (!selectedSong) {
      return;
    }

    const index = selectedSong.sections.findIndex(
      (section) => section.id === selectedSectionId
    );

    if (index === -1) {
      return;
    }

    const newIndex = index + delta;
    if (0 <= newIndex && newIndex < selectedSong.sections.length) {
      core?.sendRequest(
        selectSectionRequest(selectedSong.sections[newIndex].id)
      );
    }
  };

  const togglePlay = () => {
    if (playing) {
      core?.sendRequest(stopRequest());
    } else {
      core?.sendRequest(playRequest());
    }
  };

  const toggleLoop = () => {
    if (looping) {
      core?.sendRequest(exitLoopRequest());
    } else {
      core?.sendRequest(loopRequest());
    }
  };

  useHotkeys('space', togglePlay, {}, [core, playbackState]);
  useHotkeys('l', toggleLoop, {}, [core, playbackState]);
  useHotkeys('right', () => incrementSectionSelection(1), [
    incrementSectionSelection,
  ]);
  useHotkeys('left', () => incrementSectionSelection(-1), [
    incrementSectionSelection,
  ]);
  useHotkeys('up', () => incrementSongSelection(-1), [incrementSongSelection]);
  useHotkeys('down', () => incrementSongSelection(1), [incrementSongSelection]);

  useHotkeys('j', queueSection, [queueSection]);

  const icons = [
    {
      name: 'prev-song',
      icon: <img src={prevSongIcon} />,
      onClick: () => incrementSongSelection(-1),
    },
    {
      name: 'prev-section',
      icon: <img src={prevSectionIcon} />,
      onClick: () => incrementSectionSelection(-1),
    },
    {
      name: 'play',
      icon: playing ? <img src={stopIcon} /> : <img src={playIcon} />,
      onClick: togglePlay,
    },
    {
      name: 'next-section',
      icon: <img src={nextSectionIcon} />,
      onClick: () => incrementSectionSelection(1),
    },
    {
      name: 'next-song',
      icon: <img src={nextSongIcon} />,
      onClick: () => incrementSongSelection(1),
    },
  ];

  return (
    <div className={styles.container}>
      {playing && (
        <button
          className={`${styles['loop-button']} ${
            looping && styles['loop-button-looping']
          }`}
          onClick={toggleLoop}
        >
          <FiRepeat />
        </button>
      )}

      {icons.map((icon) => (
        <div className={styles.icon} key={icon.name} onClick={icon.onClick}>
          {icon.icon}
        </div>
      ))}

      {playing && !selectedSectionIsPlaying && (
        <button className={styles['queue-button']} onClick={queueSection}>
          {playbackState.queuedSectionId === selectedSectionId ? (
            <FiCheck />
          ) : (
            <FiCornerUpRight />
          )}
        </button>
      )}
    </div>
  );
};
