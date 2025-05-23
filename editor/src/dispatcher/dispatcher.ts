import {createContext, useContext} from 'react';
import {Action} from './action';

// The dispatcher type: a function that takes an Action
export type Dispatcher = (action: Action) => void;

// Create the dispatcher context
export const DispatcherContext = createContext<Dispatcher | null>(null);

// Hook to use the dispatcher
export const useDispatcher = () => {
  const context = useContext(DispatcherContext);
  if (!context) {
    throw new Error(
      'useDispatcher must be used within a DispatcherContext.Provider'
    );
  }
  return context;
};
