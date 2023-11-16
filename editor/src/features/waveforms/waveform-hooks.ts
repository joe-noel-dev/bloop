import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useWaveformData = (sampleId: string) =>
  useContext(CoreDataContext)?.waveforms?.get(sampleId);
