export type PlayingState = 'stopped' | 'playing';

export interface PlaybackState {
  playing: PlayingState;
  songId: string;
  sectionId: string;
  queuedSongId: string;
  queuedSectionId: string;
  loopCount: number;
  looping: boolean;
}
