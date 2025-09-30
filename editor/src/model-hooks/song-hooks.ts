import {Song} from '../api/bloop';
import {ID} from '../api/helpers';
import {useProject} from './project-hooks';

export const useSongs = () => useProject()?.songs;

export const useSong = (id: ID) =>
  useSongs()?.find((song: Song) => song.id.equals(id));

export const useSelectedSongId = () => useProject()?.selections?.song;

export const useSelectedSong = () => {
  const selectedSongId = useSelectedSongId();
  return selectedSongId && useSong(selectedSongId);
};

export const useSelectedSongIndex = () => {
  const selectedSongId = useSelectedSongId();
  return (
    selectedSongId &&
    useSongs()?.findIndex((song) => song.id.equals(selectedSongId))
  );
};
