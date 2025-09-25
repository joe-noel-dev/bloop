import {createContext, useContext} from 'react';
import {AudioController} from './AudioController';

export const AudioControllerContext = createContext<AudioController | null>(
  null
);

export const useAudioController = () => {
  const context = useContext(AudioControllerContext);
  if (!context) {
    throw new Error(
      'useAudioController must be used within an AudioControllerContext.Provider'
    );
  }
  return context;
};
