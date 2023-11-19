import {ProjectInfo} from '../model/project-info';
import {Section} from '../model/section';
import {Song} from '../model/song';
import {Sample} from '../model/sample';

export type Method =
  | 'get'
  | 'add'
  | 'select'
  | 'remove'
  | 'update'
  | 'transport'
  | 'save'
  | 'load'
  | 'rename'
  | 'beginUpload'
  | 'upload'
  | 'completeUpload'
  | 'addSample'
  | 'removeSample';

export type Entity =
  | 'all'
  | 'section'
  | 'song'
  | 'project'
  | 'projects'
  | 'waveform'
  | 'sample';

export interface Request {
  method: Method;
  payload?:
    | EntityId
    | UpdateRequest
    | TransportRequest
    | RenameRequest
    | BeginUploadRequest
    | UploadDataRequest
    | CompleteUploadRequest
    | AddSampleRequest
    | RemoveSampleRequest;
}

export interface EntityId {
  entity: Entity;
  id?: string;
}

export interface UpdateRequest {
  entity: Entity;
  value: Song | Section | ProjectInfo | Sample;
}

export interface RenameRequest {
  entity: Entity;
  id?: string;
  name: string;
}

export interface BeginUploadRequest {
  uploadId: string;
  filename: string;
  format: string;
}

export interface UploadDataRequest {
  uploadId: string;
  data: Uint8Array;
}

export interface CompleteUploadRequest {
  uploadId: string;
}

export interface AddSampleRequest {
  songId: string;
  uploadId: string;
}

export interface RemoveSampleRequest {
  songId: string;
}

export interface QueueOptions {
  songId: string;
  sectionId: string;
}

export interface TransportRequest {
  method: TransportMethod;
  options?: QueueOptions;
}

export type TransportMethod = 'play' | 'stop' | 'loop' | 'exitLoop' | 'queue';

export const getAllRequest = (): Request => {
  return {
    method: 'get',
    payload: {
      entity: 'all',
    },
  };
};

export const addSectionRequest = (songId: string): Request => {
  return {
    method: 'add',
    payload: {
      entity: 'section',
      id: songId,
    },
  };
};

export const addSongRequest = () => {
  return {
    method: 'add',
    payload: {
      entity: 'song',
    },
  };
};

export const addProjectRequest = () => {
  return {
    method: 'add',
    payload: {
      entity: 'project',
    },
  };
};

export const selectSongRequest = (songId: string) => {
  return {
    method: 'select',
    payload: {
      entity: 'song',
      id: songId,
    },
  };
};

export const selectSectionRequest = (sectionId: string): Request => {
  return {
    method: 'select',
    payload: {
      entity: 'section',
      id: sectionId,
    },
  };
};

export const removeRequest = (entity: Entity, id: string): Request => {
  return {
    method: 'remove',
    payload: {
      entity,
      id,
    },
  };
};

export const removeSongRequest = (songId: string) =>
  removeRequest('song', songId);

export const removeSectionRequest = (sectionId: string) =>
  removeRequest('section', sectionId);

export const removeProjectRequest = (projectId: string) =>
  removeRequest('project', projectId);

export const removeSampleRequest = (songId: string): Request => {
  return {
    method: 'removeSample',
    payload: {
      songId,
    },
  };
};

export const updateRequest = (
  entity: Entity,
  value: Song | Section | ProjectInfo | Sample
): Request => {
  return {
    method: 'update',
    payload: {
      entity,
      value,
    },
  };
};

export const updateSongRequest = (song: Song): Request =>
  updateRequest('song', song);

export const updateSectionRequest = (section: Section): Request =>
  updateRequest('section', section);

export const updateSampleRequest = (sample: Sample): Request =>
  updateRequest('sample', sample);

const transportRequest = (method: TransportMethod): Request => {
  return {
    method: 'transport',
    payload: {
      method,
    },
  };
};

export const playRequest = (): Request => transportRequest('play');
export const stopRequest = (): Request => transportRequest('stop');
export const loopRequest = (): Request => transportRequest('loop');
export const exitLoopRequest = (): Request => transportRequest('exitLoop');

export const queueRequest = (songId: string, sectionId: string): Request => {
  return {
    method: 'transport',
    payload: {
      method: 'queue',
      options: {songId, sectionId},
    },
  };
};

export const saveRequest = (): Request => {
  return {
    method: 'save',
  };
};

export const loadProjectsRequest = (): Request => {
  return {
    method: 'get',
    payload: {
      entity: 'projects',
    },
  };
};

export const loadProjectRequest = (projectId: string): Request => {
  return {
    method: 'load',
    payload: {
      entity: 'project',
      id: projectId,
    },
  };
};

export const renameProjectRequest = (name: string): Request => {
  return {
    method: 'rename',
    payload: {
      entity: 'project',
      name,
    },
  };
};

export const beginUploadRequest = (
  uploadId: string,
  filename: string,
  format: string
): Request => {
  return {
    method: 'beginUpload',
    payload: {
      uploadId,
      filename,
      format,
    },
  };
};

export const uploadRequest = (uploadId: string, data: ArrayBuffer): Request => {
  return {
    method: 'upload',
    payload: {
      uploadId,
      data: new Uint8Array(data),
    },
  };
};

export const completeUploadRequest = (uploadId: string): Request => {
  return {
    method: 'completeUpload',
    payload: {
      uploadId,
    },
  };
};

export const addSampleRequest = (songId: string, uploadId: string): Request => {
  return {
    method: 'addSample',
    payload: {
      songId,
      uploadId,
    },
  };
};

export const requestWaveformRequest = (sampleId: string): Request => {
  return {
    method: 'get',
    payload: {
      entity: 'waveform',
      id: sampleId,
    },
  };
};
