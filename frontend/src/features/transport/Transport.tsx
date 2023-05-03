import React from 'react';
import styled from 'styled-components';
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

    const index = selectedSong.sectionIds.indexOf(selectedSectionId || '');
    if (index === -1) {
      return;
    }

    const newIndex = index + delta;
    if (0 <= newIndex && newIndex < selectedSong.sectionIds.length) {
      core?.sendRequest(
        selectSectionRequest(selectedSong.sectionIds[newIndex])
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
    <Container>
      {playing && (
        <LoopIcon looping={looping} onClick={toggleLoop}>
          <FiRepeat size={16} />
        </LoopIcon>
      )}

      {icons.map((icon) => (
        <Icon key={icon.name} onClick={icon.onClick}>
          {icon.icon}
        </Icon>
      ))}

      {playing && !selectedSectionIsPlaying && (
        <QueueButton onClick={queueSection}>
          {playbackState.queuedSectionId === selectedSectionId ? (
            <FiCheck size={16} />
          ) : (
            <FiCornerUpRight size={16} />
          )}
        </QueueButton>
      )}
    </Container>
  );
};

const Container = styled.div`
  height: ${(props) => props.theme.units(10)};

  background: ${(props) => props.theme.colours.primaryLight};

  box-shadow: ${(props) => props.theme.dropShadow};

  display: flex;
  align-items: center;
  justify-content: center;

  position: relative;
`;

const QueueButton = styled.button`
  position: absolute;
  right: ${(props) => props.theme.units(2)};
  top: 50%;
  transform: translate(0, -50%);
  width: ${(props) => props.theme.units(6)};
  height: ${(props) => props.theme.units(6)};

  border-radius: 50%;
  background: #2e2e2e;

  color: white;
`;

const Icon = styled.div`
  margin: 0 1rem;
  fill: ${(props) => props.theme.textColours.primary};
`;

interface LoopIconProps {
  looping: boolean;
}

const LoopIcon = styled.button<LoopIconProps>`
  position: absolute;
  left: ${(props) => props.theme.units(2)};
  top: 50%;
  transform: translate(0, -50%);

  width: ${(props) => props.theme.units(6)};
  height: ${(props) => props.theme.units(6)};

  border-radius: 50%;
  background: #2e2e2e;

  color: white;
  opacity: ${(props) => (props.looping ? '100%' : '40%')};
`;
