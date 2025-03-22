import {Song} from '../api/bloop';
import {ID, INVALID_ID} from '../api/helpers';
import {useProject} from './project-hooks';

export const useSongs = () => useProject()?.songs;

export const useSong = (id: ID) =>
  useSongs()?.find((song: Song) => song.id === id);

export const useSelectedSongId = () => useProject()?.selections?.song;

export const useSelectedSong = () => {
  const selectedSongId = useSelectedSongId();
  return useSong(selectedSongId || INVALID_ID);
};

export const useSelectedSongIndex = () => {
  const selectedSongId = useSelectedSongId();
  return useSongs()?.findIndex((song) => song.id === selectedSongId);
};
