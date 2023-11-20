import {useCoreData} from '../core/CoreData';

export const usePlaybackState = () => useCoreData().playbackState;

export const useProgress = () => useCoreData().progress;
