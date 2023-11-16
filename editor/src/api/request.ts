import {ProjectInfo} from '../model/project-info';
import {Section} from '../model/section';
import {Song} from '../model/song';
import {Sample} from '../model/sample';

export enum Method {
  get = 'get',
  add = 'add',
  select = 'select',
  remove = 'remove',
  update = 'update',
  transport = 'transport',
  save = 'save',
  load = 'load',
  rename = 'rename',
  beginUpload = 'beginUpload',
  upload = 'upload',
  completeUpload = 'completeUpload',
  addSample = 'addSample',
  removeSample = 'removeSample',
}

export enum Entity {
  all = 'all',
  section = 'section',
  song = 'song',
  project = 'project',
  projects = 'projects',
  waveform = 'waveform',
  sample = 'sample',
}

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

export enum TransportMethod {
  play = 'play',
  stop = 'stop',
  loop = 'loop',
  exitLoop = 'exitLoop',
  queue = 'queue',
}

export function getAllRequest(): Request {
  return {
    method: Method.get,
    payload: {
      entity: Entity.all,
    },
  };
}

export function addSectionRequest(songId: string): Request {
  return {
    method: Method.add,
    payload: {
      entity: Entity.section,
      id: songId,
    },
  };
}

export function addSongRequest(): Request {
  return {
    method: Method.add,
    payload: {
      entity: Entity.song,
    },
  };
}

export function addProjectRequest(): Request {
  return {
    method: Method.add,
    payload: {
      entity: Entity.project,
    },
  };
}

export function selectSongRequest(songId: string): Request {
  return {
    method: Method.select,
    payload: {
      entity: Entity.song,
      id: songId,
    },
  };
}

export function selectSectionRequest(sectionId: string): Request {
  return {
    method: Method.select,
    payload: {
      entity: Entity.section,
      id: sectionId,
    },
  };
}

export function removeRequest(entity: Entity, id: string): Request {
  return {
    method: Method.remove,
    payload: {
      entity,
      id,
    },
  };
}

export function removeSongRequest(songId: string): Request {
  return removeRequest(Entity.song, songId);
}

export function removeSectionRequest(sectionId: string): Request {
  return removeRequest(Entity.section, sectionId);
}

export function removeProjectRequest(projectId: string): Request {
  return removeRequest(Entity.project, projectId);
}

export function removeSampleRequest(songId: string): Request {
  return {
    method: Method.removeSample,
    payload: {
      songId,
    },
  };
}

export function updateRequest(
  entity: Entity,
  value: Song | Section | ProjectInfo | Sample
): Request {
  return {
    method: Method.update,
    payload: {
      entity,
      value,
    },
  };
}

export function updateSongRequest(song: Song): Request {
  return updateRequest(Entity.song, song);
}

export function updateSectionRequest(section: Section): Request {
  return updateRequest(Entity.section, section);
}

export function updateSampleRequest(sample: Sample): Request {
  return updateRequest(Entity.sample, sample);
}

function transportRequest(method: TransportMethod): Request {
  return {
    method: Method.transport,
    payload: {
      method,
    },
  };
}

export function playRequest(): Request {
  return transportRequest(TransportMethod.play);
}

export function stopRequest(): Request {
  return transportRequest(TransportMethod.stop);
}

export function loopRequest(): Request {
  return transportRequest(TransportMethod.loop);
}

export function exitLoopRequest(): Request {
  return transportRequest(TransportMethod.exitLoop);
}

export function queueRequest(songId: string, sectionId: string) {
  return {
    method: Method.transport,
    payload: {
      method: TransportMethod.queue,
      options: {songId, sectionId},
    },
  };
}

export function saveRequest(): Request {
  return {
    method: Method.save,
  };
}

export function loadProjectsRequest(): Request {
  return {
    method: Method.get,
    payload: {
      entity: Entity.projects,
    },
  };
}

export function loadProjectRequest(projectId: string): Request {
  return {
    method: Method.load,
    payload: {
      entity: Entity.project,
      id: projectId,
    },
  };
}

export function renameProjectRequest(name: string): Request {
  return {
    method: Method.rename,
    payload: {
      entity: Entity.project,
      name,
    },
  };
}

export function beginUploadRequest(
  uploadId: string,
  filename: string,
  format: string
): Request {
  return {
    method: Method.beginUpload,
    payload: {
      uploadId,
      filename,
      format,
    },
  };
}

export function uploadRequest(uploadId: string, data: ArrayBuffer): Request {
  return {
    method: Method.upload,
    payload: {
      uploadId,
      data: new Uint8Array(data),
    },
  };
}

export function completeUploadRequest(uploadId: string): Request {
  return {
    method: Method.completeUpload,
    payload: {
      uploadId,
    },
  };
}

export function addSampleRequest(songId: string, uploadId: string): Request {
  return {
    method: Method.addSample,
    payload: {
      songId,
      uploadId,
    },
  };
}

export function requestWaveformRequest(sampleId: string): Request {
  return {
    method: Method.get,
    payload: {
      entity: Entity.waveform,
      id: sampleId,
    },
  };
}
