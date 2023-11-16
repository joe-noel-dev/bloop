import {Project} from '../model/project';
import {ProjectInfo} from '../model/project-info';
import {PlaybackState} from '../model/playback-state';
import {Waveform} from '../model/waveform';
import {Progress} from '../model/progress';

interface UploadAck {
  uploadId: string;
}

export interface Response {
  project?: Project;
  playbackState?: PlaybackState;
  progress?: Progress;
  error?: string;
  projects?: ProjectInfo[];
  upload?: UploadAck;
  waveform?: Waveform;
}
