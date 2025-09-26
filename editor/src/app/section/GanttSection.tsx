import React, {useState, useRef, useCallback} from 'react';
import {Box, Stack, IconButton, Switch} from '@mui/joy';
import {
  PlayArrow,
  Stop,
  Delete,
} from '@mui/icons-material';
import {useSectionById} from '../../model-hooks/section-hooks';
import {ClickToEdit} from '../../components/ClickToEdit';
import {ID} from '../../api/helpers';
import {Section as ModelSection} from '../../api/bloop';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {
  playAction,
  stopAction,
  updateSectionAction,
  removeSectionAction,
} from '../../dispatcher/action';
import {useAppState} from '../../state/AppState';
import isEqual from 'lodash.isequal';

interface GanttSectionProps {
  songId: ID;
  sectionId: ID;
  startTime: number;
  duration: number;
  totalDuration: number;
  requestUpdateDuration: (sectionId: ID, duration: number) => void;
  onSectionStartChange: (sectionId: ID, newStart: number) => void;
}

const SECTION_HEIGHT = 40;
const HANDLE_WIDTH = 8;

export const GanttSection = ({
  songId,
  sectionId,
  startTime,
  duration,
  totalDuration,
  requestUpdateDuration,
  onSectionStartChange,
}: GanttSectionProps) => {
  const section = useSectionById(sectionId);
  const state = useAppState();
  const dispatch = useDispatcher();
  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [dragStart, setDragStart] = useState({x: 0, time: 0});
  const sectionRef = useRef<HTMLDivElement>(null);

  if (!section) {
    return null;
  }

  const updateSection = (section: ModelSection) =>
    dispatch(updateSectionAction(songId, section));

  const isPlaying =
    state.playing &&
    state.playingSongId?.equals(songId) &&
    state.playingSectionId?.equals(sectionId);

  const handlePlay = () => {
    dispatch(playAction(songId, sectionId, section.loop));
  };

  const handleStop = () => {
    dispatch(stopAction());
  };

  const submitName = (name: string) => {
    const newSection = {...section, name};
    if (isEqual(section, newSection)) {
      return;
    }
    updateSection(newSection);
  };

  const enableLoop = (enable: boolean) =>
    updateSection({...section, loop: enable});

  const enableMetronome = (enable: boolean) =>
    updateSection({...section, metronome: enable});

  const remove = () => dispatch(removeSectionAction(songId, sectionId));

  // Calculate positioning
  const leftPercent = totalDuration > 0 ? (startTime / totalDuration) * 100 : 0;
  const widthPercent = totalDuration > 0 ? (duration / totalDuration) * 100 : 100;

  const handleMouseDown = useCallback(
    (e: React.MouseEvent, type: 'drag' | 'resize-left' | 'resize-right') => {
      e.preventDefault();
      e.stopPropagation();
      
      if (type === 'drag') {
        setIsDragging(true);
        setDragStart({x: e.clientX, time: startTime});
      } else {
        setIsResizing(true);
        setDragStart({x: e.clientX, time: type === 'resize-left' ? startTime : startTime + duration});
      }

      const handleMouseMove = (e: MouseEvent) => {
        if (!sectionRef.current) return;

        const containerRect = sectionRef.current.parentElement?.getBoundingClientRect();
        if (!containerRect) return;

        const pixelsPerBeat = containerRect.width / totalDuration;
        const deltaX = e.clientX - dragStart.x;
        const deltaTime = deltaX / pixelsPerBeat;

        if (type === 'drag') {
          const newStart = Math.max(0, Math.min(totalDuration - duration, dragStart.time + deltaTime));
          onSectionStartChange(sectionId, newStart);
        } else if (type === 'resize-right') {
          const newDuration = Math.max(0.1, dragStart.time - startTime + deltaTime);
          requestUpdateDuration(sectionId, newDuration);
        } else if (type === 'resize-left') {
          const newStart = Math.max(0, Math.min(dragStart.time + deltaTime, startTime + duration - 0.1));
          const newDuration = duration + (startTime - newStart);
          onSectionStartChange(sectionId, newStart);
          requestUpdateDuration(sectionId, newDuration);
        }
      };

      const handleMouseUp = () => {
        setIsDragging(false);
        setIsResizing(false);
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };

      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    },
    [startTime, duration, totalDuration, dragStart, sectionId, requestUpdateDuration, onSectionStartChange]
  );

  return (
    <Box
      ref={sectionRef}
      sx={{
        position: 'absolute',
        left: `${leftPercent}%`,
        width: `${widthPercent}%`,
        height: SECTION_HEIGHT,
        backgroundColor: isPlaying ? 'success.200' : 'primary.100',
        border: '2px solid',
        borderColor: isPlaying ? 'success.500' : 'primary.300',
        borderRadius: 'sm',
        cursor: isDragging ? 'grabbing' : 'grab',
        transition: isDragging || isResizing ? 'none' : 'all 0.2s ease',
        opacity: isDragging || isResizing ? 0.8 : 1,
        zIndex: isDragging || isResizing ? 1000 : 1,
        display: 'flex',
        alignItems: 'center',
        minWidth: '60px',
        '&:hover': {
          borderColor: isPlaying ? 'success.700' : 'primary.500',
          boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
        },
      }}
      onMouseDown={(e) => handleMouseDown(e, 'drag')}
    >
      {/* Left resize handle */}
      <Box
        sx={{
          position: 'absolute',
          left: -HANDLE_WIDTH / 2,
          top: 0,
          bottom: 0,
          width: HANDLE_WIDTH,
          backgroundColor: 'primary.500',
          cursor: 'ew-resize',
          opacity: 0.7,
          '&:hover': {opacity: 1},
        }}
        onMouseDown={(e) => handleMouseDown(e, 'resize-left')}
      />

      {/* Section content */}
      <Stack
        direction="row"
        spacing={0.5}
        alignItems="center"
        sx={{
          flex: 1,
          paddingX: 1,
          minWidth: 0,
          height: '100%',
        }}
      >
        {/* Transport control */}
        <IconButton
          size="sm"
          variant="soft"
          color={isPlaying ? 'neutral' : 'success'}
          onClick={(e) => {
            e.stopPropagation();
            isPlaying ? handleStop() : handlePlay();
          }}
          sx={{flexShrink: 0}}
        >
          {isPlaying ? <Stop /> : <PlayArrow />}
        </IconButton>

        {/* Name - only show if there's enough space */}
        <Box sx={{flex: 1, minWidth: 0, overflow: 'hidden'}}>
          <ClickToEdit
            initialValue={section.name}
            onSave={submitName}
            size="small"
          />
        </Box>

        {/* Controls - only show if there's enough space */}
        <Stack direction="row" spacing={0.25} sx={{flexShrink: 0}}>
          <Switch
            size="sm"
            checked={section.loop}
            onChange={(event) => {
              event.stopPropagation();
              enableLoop(event.target.checked);
            }}
            sx={{
              '& .MuiSwitch-thumb': {
                width: 12,
                height: 12,
              },
              '& .MuiSwitch-track': {
                height: 16,
                width: 24,
              },
            }}
          />
          <Switch
            size="sm"
            checked={section.metronome}
            onChange={(event) => {
              event.stopPropagation();
              enableMetronome(event.target.checked);
            }}
            sx={{
              '& .MuiSwitch-thumb': {
                width: 12,
                height: 12,
              },
              '& .MuiSwitch-track': {
                height: 16,
                width: 24,
              },
            }}
          />
          <IconButton
            size="sm"
            variant="soft"
            color="danger"
            onClick={(e) => {
              e.stopPropagation();
              remove();
            }}
          >
            <Delete />
          </IconButton>
        </Stack>
      </Stack>

      {/* Right resize handle */}
      <Box
        sx={{
          position: 'absolute',
          right: -HANDLE_WIDTH / 2,
          top: 0,
          bottom: 0,
          width: HANDLE_WIDTH,
          backgroundColor: 'primary.500',
          cursor: 'ew-resize',
          opacity: 0.7,
          '&:hover': {opacity: 1},
        }}
        onMouseDown={(e) => handleMouseDown(e, 'resize-right')}
      />
    </Box>
  );
};