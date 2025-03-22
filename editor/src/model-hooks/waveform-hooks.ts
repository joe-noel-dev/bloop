import {ID} from '../api/helpers';
import {useCoreData} from '../core/CoreData';

export const useWaveformData = (sampleId: ID) =>
  useCoreData()?.waveforms?.get(sampleId);
