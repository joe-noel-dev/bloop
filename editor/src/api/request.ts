import {
  AudioFileFormat,
  Entity,
  Project,
  Request,
  Sample,
  Section,
  Song,
  TransportMethod,
} from './bloop';

import {ID, INVALID_ID} from './helpers';

export const getAllRequest = (): Request => {
  return {
    get: {
      entity: Entity.ALL,
      id: INVALID_ID,
    },
  };
};

export const addSectionRequest = (songId: ID): Request => {
  return {
    add: {
      entity: Entity.SECTION,
      id: songId,
    },
  };
};

export const addSectionWithParamsRequest = (
  songId: ID,
  name: string,
  start: number
): Request => {
  return {
    addSection: {
      songId,
      name,
      start,
      loop: false,
      metronome: false,
    },
  };
};

export const addSongRequest = (): Request => {
  return {
    add: {
      entity: Entity.SONG,
      id: INVALID_ID,
    },
  };
};

export const addProjectRequest = (): Request => {
  return {
    add: {
      entity: Entity.PROJECT,
      id: INVALID_ID,
    },
  };
};

export const selectSongRequest = (songId: ID): Request => {
  return {
    select: {
      entity: Entity.SONG,
      id: songId,
    },
  };
};

export const selectSectionRequest = (sectionId: ID): Request => {
  return {
    select: {
      entity: Entity.SECTION,
      id: sectionId,
    },
  };
};

export const removeRequest = (entity: Entity, id: ID): Request => {
  return {
    remove: {
      entity,
      id,
    },
  };
};

export const removeSongRequest = (songId: ID) =>
  removeRequest(Entity.SONG, songId);

export const removeSectionRequest = (sectionId: ID) =>
  removeRequest(Entity.SECTION, sectionId);

export const removeProjectRequest = (projectId: ID) =>
  removeRequest(Entity.PROJECT, projectId);

export const removeSampleRequest = (songId: ID): Request => {
  return {
    removeSample: {
      songId,
    },
  };
};

export const updateSongRequest = (song: Song): Request => {
  return {
    update: {
      song,
    },
  };
};

export const updateSectionRequest = (section: Section): Request => {
  return {
    update: {
      section,
    },
  };
};

export const updateSampleRequest = (sample: Sample): Request => {
  return {
    update: {
      sample,
    },
  };
};

export const updateProjectRequest = (project: Project): Request => {
  return {
    update: {
      project,
    },
  };
};

const transportRequest = (method: TransportMethod): Request => {
  return {
    transport: {
      method,
    },
  };
};

export const playRequest = (): Request =>
  transportRequest(TransportMethod.PLAY);
export const stopRequest = (): Request =>
  transportRequest(TransportMethod.STOP);
export const loopRequest = (): Request =>
  transportRequest(TransportMethod.LOOP);
export const exitLoopRequest = (): Request =>
  transportRequest(TransportMethod.EXIT_LOOP);

export const queueRequest = (songId: ID, sectionId: ID): Request => {
  return {
    transport: {
      method: TransportMethod.QUEUE,
      queue: {songId, sectionId},
    },
  };
};

export const saveRequest = (): Request => {
  return {
    save: {},
  };
};

export const loadProjectsRequest = (): Request => {
  return {
    get: {
      entity: Entity.PROJECTS,
      id: INVALID_ID,
    },
  };
};

export const loadProjectRequest = (projectId: string): Request => {
  return {
    load: {
      projectId,
    },
  };
};

export const renameProjectRequest = (
  projectId: string,
  name: string
): Request => {
  return {
    renameProject: {
      projectId,
      newName: name,
    },
  };
};

export const beginUploadRequest = (
  uploadId: ID,
  filename: string,
  format: AudioFileFormat
): Request => {
  return {
    beginUpload: {
      uploadId,
      filename,
      format,
    },
  };
};

export const uploadRequest = (uploadId: ID, data: ArrayBuffer): Request => {
  return {
    upload: {
      uploadId,
      data: new Uint8Array(data),
    },
  };
};

export const completeUploadRequest = (uploadId: ID): Request => {
  return {
    completeUpload: {
      uploadId,
    },
  };
};

export const addSampleRequest = (songId: ID, uploadId: ID): Request => {
  return {
    addSample: {
      songId,
      uploadId,
    },
  };
};

export const requestWaveformRequest = (sampleId: ID): Request => {
  return {
    get: {
      entity: Entity.SAMPLE,
      id: sampleId,
    },
  };
};
