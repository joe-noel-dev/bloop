import {createContext, useContext} from 'react';

type Context = [string, (sectionId: string) => void];

export const EditingSectionContext = createContext<Context>(['', () => {}]);

export const useEditingSection = () => {
  const context = useContext(EditingSectionContext);

  if (!context) {
    throw new Error(
      'useEditingSection should be called from within a EditingSectionContext provider'
    );
  }

  return context;
};
