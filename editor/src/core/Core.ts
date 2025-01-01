import {getAllRequest, Request} from '../api/request';
import {Response} from '../api/response';
import {serialize, deserialize, setInternalBufferSize, Long} from 'bson';
import {WaveformData} from '../model/waveform';
import {EventEmitter} from 'events';
import {ProjectInfo} from '../model/project-info';

const logRequests: boolean = import.meta.env.VITE_BLOOP_LOG_REQUESTS === 'true';

const SERIALISATION_BUFFER_SIZE = 100 * 1024 * 1024;
setInternalBufferSize(SERIALISATION_BUFFER_SIZE);

export interface CoreInstance {
  sendRequest(request: Request): void;
  waitForUploadAck(uploadId: string): Promise<void>;
}

export const createCore = () => {
  let socket: null | WebSocket = null;

  let uploadPromises: {[uploadId: string]: () => void} = {};
  let waitingAcks: string[] = [];
  let pendingRequests: Request[] = [];

  let waveforms = new Map<string, WaveformData>();

  const eventEmitter = new EventEmitter();

  const sendRequest = (request: Request) => {
    if (socket && socket.readyState === WebSocket.OPEN) {
      if (logRequests) {
        console.log('Sending: ', request);
      }

      socket?.send(serialize(request));
    } else {
      pendingRequests.push(request);
    }
  };

  const connect = (address: string) => {
    if (!socket) {
      console.log('Connecting to ', address);
      socket = new WebSocket(address);

      socket.binaryType = 'arraybuffer';

      socket.onmessage = onMessage;

      socket.onopen = onOpen;
      socket.onclose = onClose;
      socket.onerror = onError;
    }
  };

  const onMessage = (event: MessageEvent) => {
    try {
      const message: Response = deserialize(event.data);

      if (message.project) {
        fixLastSaved(message.project.info);
        eventEmitter.emit('project', message.project);
      }

      if (message.playbackState) {
        eventEmitter.emit('playback-state', message.playbackState);
      }

      if (message.progress) {
        eventEmitter.emit('progress', message.progress);
      }

      if (message.projects) {
        message.projects.forEach(fixLastSaved);
        eventEmitter.emit('projects', message.projects);
      }

      if (message.waveform) {
        console.info(
          `Received waveform data for sample ${message.waveform.sampleId}`
        );
        waveforms.set(message.waveform.sampleId, message.waveform.waveformData);

        eventEmitter.emit('waveforms', waveforms);
      }

      if (message.upload) {
        const uploadId = message.upload.uploadId;
        if (uploadPromises[uploadId]) {
          uploadPromises[uploadId]();
        } else {
          waitingAcks.push(uploadId);
        }
      }

      if (message.error) {
        console.error(`Error from core: ${message.error}`);
      }
    } catch (error) {
      console.error(`Unable to parse response`);
    }
  };

  const onOpen = () => {
    eventEmitter.emit('connect');
    sendRequest(getAllRequest());

    pendingRequests.forEach(sendRequest);
    pendingRequests = [];

    console.log('Connected to core');
  };

  const onClose = () => {
    eventEmitter.emit('disconnect');
    socket = null;
    console.log('Disconnected from core');
  };

  const onError = () => {
    socket?.close();
    socket = null;
  };

  const disconnect = () => {
    socket?.close();
  };

  const waitForUploadAck = (uploadId: string) => {
    return new Promise<void>((resolve) => {
      if (waitingAcks.find((id) => id === uploadId)) {
        resolve();
      } else {
        uploadPromises[uploadId] = resolve;
      }
    });
  };

  return {
    sendRequest,
    waitForUploadAck,
    disconnect,
    connect,
    events: eventEmitter,
  };
};

export type Core = ReturnType<typeof createCore>;

const fixLastSaved = (projectInfo: ProjectInfo) => {
  // Hack to workaround we receive lastSaved as number instead of Long, but
  // the core expects an i64
  if (typeof projectInfo.lastSaved === 'number') {
    projectInfo.lastSaved = Long.fromNumber(projectInfo.lastSaved as number);
  }
};
