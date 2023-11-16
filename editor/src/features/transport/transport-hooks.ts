import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const usePlaybackState = () => useContext(CoreDataContext).playbackState;

export const useProgress = () => useContext(CoreDataContext).progress;
