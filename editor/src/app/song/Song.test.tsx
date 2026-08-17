import {describe, expect, it, vi} from 'vitest';
import {render, screen} from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Long from 'long';
import {Song} from './Song';
import {AppStateContext} from '../../state/AppState';
import {DispatcherContext} from '../../dispatcher/dispatcher';
import {emptyProject} from '../../api/project-helpers';
import {createTestAppStateWithProject} from '../../test-utils/app-state-helpers';

vi.mock('@mui/icons-material', () => ({
  Add: () => <svg data-testid="add-icon" />,
  ArrowDownward: () => <svg data-testid="arrow-down-icon" />,
  ArrowUpward: () => <svg data-testid="arrow-up-icon" />,
  Delete: () => <svg data-testid="delete-icon" />,
  DragIndicator: () => <svg data-testid="drag-icon" />,
  Download: () => <svg data-testid="download-icon" />,
  Error: () => <svg data-testid="error-icon" />,
  FileUpload: () => <svg data-testid="file-upload-icon" />,
  Sync: () => <svg data-testid="sync-icon" />,
  Upload: () => <svg data-testid="upload-icon" />,
}));

const songId = Long.fromNumber(123);
const sampleId = Long.fromNumber(456);

const renderSong = (sampleName?: string, volume?: number) => {
  const dispatch = vi.fn();
  const project = emptyProject();
  project.songs = [
    {
      id: songId,
      name: 'Main Track',
      tempo: {bpm: 128},
      volume,
      sections: [],
      sample: sampleName
        ? {
            id: sampleId,
            name: sampleName,
            tempo: {bpm: 128},
            sampleRate: 44100,
            sampleCount: Long.fromNumber(1000),
            channelCount: 2,
          }
        : undefined,
    },
  ];

  render(
    <AppStateContext.Provider value={createTestAppStateWithProject(project)}>
      <DispatcherContext.Provider value={dispatch}>
        <Song songId={songId} moveSong={() => {}} />
      </DispatcherContext.Provider>
    </AppStateContext.Provider>
  );

  return dispatch;
};

describe('Song', () => {
  it('shows the sample name in the track details', () => {
    renderSong('breakbeat-128bpm.wav');

    expect(screen.getByText('Sample')).toBeInTheDocument();
    expect(screen.getByText('breakbeat-128bpm.wav')).toBeInTheDocument();
  });

  it('shows an empty sample state before a sample is selected', () => {
    renderSong();

    expect(screen.getByText('No sample selected')).toBeInTheDocument();
  });

  it('shows the default unity volume', () => {
    renderSong();

    expect(screen.getByText('Volume')).toBeInTheDocument();
    expect(screen.getByLabelText('Click to edit 0')).toBeInTheDocument();
  });

  it('updates the song volume in decibels', async () => {
    const dispatch = renderSong();
    const user = userEvent.setup();

    await user.click(screen.getByLabelText('Click to edit 0'));
    await user.clear(screen.getByRole('textbox'));
    await user.type(screen.getByRole('textbox'), '-6');
    await user.keyboard('{Enter}');

    expect(dispatch).toHaveBeenCalledWith(
      expect.objectContaining({
        type: 'UPDATE_SONG',
        newSong: expect.objectContaining({volume: -6}),
      })
    );
  });
});
