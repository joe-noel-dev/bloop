import {useRef, useEffect} from 'react';
import {Progress} from './AudioController';

/**
 * ProgressService provides direct access to progress data without triggering React re-renders.
 * This is more efficient for high-frequency updates (like every 15ms) that only need to update
 * specific UI elements rather than the entire app state.
 */
export class ProgressService {
  private progressRef = {current: null as Progress | null};
  private listeners = new Set<(progress: Progress | null) => void>();

  updateProgress(progress: Progress | null) {
    this.progressRef.current = progress;
    // Only notify listeners, don't trigger state updates
    this.listeners.forEach((listener) => listener(progress));
  }

  getCurrentProgress(): Progress | null {
    return this.progressRef.current;
  }

  subscribe(listener: (progress: Progress | null) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
}

// Global instance for the app
export const progressService = new ProgressService();

/**
 * Hook to subscribe to progress updates without causing re-renders of parent components.
 * Use this for UI elements that need real-time progress updates.
 */
export const useProgressSubscription = (
  callback: (progress: Progress | null) => void
) => {
  const callbackRef = useRef(callback);
  callbackRef.current = callback;

  useEffect(() => {
    const unsubscribe = progressService.subscribe((progress) => {
      callbackRef.current(progress);
    });

    return unsubscribe;
  }, []);
};

/**
 * Hook to get current progress value without subscribing to updates.
 * Useful for one-time reads or when you control when to check for updates.
 */
export const useCurrentProgress = (): Progress | null => {
  return progressService.getCurrentProgress();
};
