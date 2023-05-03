import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useSongs = () => useContext(CoreDataContext).project?.songs;

export const useSong = (id: string) =>
  useSongs()?.find((song) => song.id === id);

export const useSelectedSongId = () =>
  useContext(CoreDataContext).project?.selections.song;

export const useSelectedSong = () => {
  const selectedSongId = useSelectedSongId();
  return useSong(selectedSongId || '');
};

export const useSelectedSongIndex = () => {
  const selectedSongId = useSelectedSongId();
  return useSongs()?.findIndex((song) => song.id === selectedSongId);
};
