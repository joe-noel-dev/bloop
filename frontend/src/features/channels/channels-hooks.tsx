import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useChannels = () => useContext(CoreDataContext).project?.channels;

export const useChannel = (id: string) =>
  useChannels()?.find((channel) => channel.id === id);
