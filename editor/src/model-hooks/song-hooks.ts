import {useProject} from './project-hooks';

export const useSongs = () => useProject()?.songs;

export const useSong = (id: string) =>
  useSongs()?.find((song) => song.id === id);

export const useSelectedSongId = () => useProject()?.selections.song;

export const useSelectedSong = () => {
  const selectedSongId = useSelectedSongId();
  return useSong(selectedSongId || '');
};

export const useSelectedSongIndex = () => {
  const selectedSongId = useSelectedSongId();
  return useSongs()?.findIndex((song) => song.id === selectedSongId);
};
