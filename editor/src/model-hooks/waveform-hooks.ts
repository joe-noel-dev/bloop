import {useCoreData} from '../core/CoreData';

export const useWaveformData = (sampleId: string) =>
  useCoreData()?.waveforms?.get(sampleId);
