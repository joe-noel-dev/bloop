import {getAllRequest} from '../api/request';
import {EventEmitter} from 'events';
import {Request, Response, WaveformData} from '../api/bloop';
import {ID} from '../api/helpers';

const logRequests: boolean = import.meta.env.VITE_BLOOP_LOG_REQUESTS === 'true';

export interface CoreInstance {
  sendRequest(request: Request): void;
  waitForUploadAck(uploadId: string): Promise<void>;
}

export const createCore = () => {
  let socket: null | WebSocket = null;

  let uploadPromises = new Map<string, () => void>();
  let waitingAcks: ID[] = [];
  let pendingRequests: Request[] = [];

  let waveforms = new Map<ID, WaveformData>();

  const eventEmitter = new EventEmitter();

  const sendRequest = (request: Request) => {
    if (socket && socket.readyState === WebSocket.OPEN) {
      if (logRequests) {
        console.log('Sending: ', request);
      }

      let data = Request.encode(request).finish();
      socket?.send(data);
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
      const message: Response = Response.decode(new Uint8Array(event.data));

      if (message.project) {
        eventEmitter.emit('project', message.project);
      }

      if (message.playbackState) {
        eventEmitter.emit('playback-state', message.playbackState);
      }

      if (message.progress) {
        eventEmitter.emit('progress', message.progress);
      }

      if (message.projects) {
        eventEmitter.emit('projects', message.projects);
      }

      if (message.waveform && message.waveform.waveformData) {
        console.info(
          `Received waveform data for sample ${message.waveform.sampleId}`
        );
        waveforms.set(message.waveform.sampleId, message.waveform.waveformData);

        eventEmitter.emit('waveforms', waveforms);
      }

      if (message.upload) {
        console.debug(`Received upload ack for ${message.upload.uploadId}`);
        const uploadId = message.upload.uploadId;
        const action = uploadPromises.get(uploadId.toString());
        if (action) {
          action();
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

  const waitForUploadAck = (uploadId: ID) => {
    return new Promise<void>((resolve) => {
      if (waitingAcks.find((id) => id === uploadId)) {
        resolve();
      } else {
        uploadPromises.set(uploadId.toString(), resolve);
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
