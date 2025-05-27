import {createContext, useContext} from 'react';
import {Action} from './action';

export type Dispatcher = (action: Action) => void;

export const DispatcherContext = createContext<Dispatcher | null>(null);

export const useDispatcher = () => {
  const context = useContext(DispatcherContext);
  if (!context) {
    throw new Error(
      'useDispatcher must be used within a DispatcherContext.Provider'
    );
  }
  return context;
};
