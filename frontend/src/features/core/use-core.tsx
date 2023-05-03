import {createContext, useContext} from 'react';
import {Core} from './Core';

export const CoreContext = createContext<Core | null>(null);

export const useCore = () => {
  const context = useContext(CoreContext);
  if (!context) {
    throw new Error('useCore should be called from within a CoreProvider');
  }
  return context;
};
