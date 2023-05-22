import config from '../../config';
import {getAllRequest, Request} from '../../api/request';
import {Response} from '../../api/response';
import {serialize, deserialize, setInternalBufferSize} from 'bson';
import {Project} from '../../model/project';
import {PlaybackState} from '../../model/playback-state';
import {Progress} from '../../model/progress';
import {ProjectInfo} from '../../model/project-info';
import {WaveformData} from '../../model/waveform';

const SERIALISATION_BUFFER_SIZE = 100 * 1024 * 1024;
setInternalBufferSize(SERIALISATION_BUFFER_SIZE);

export interface CoreInstance {
  sendRequest(request: Request): void;
  waitForUploadAck(uploadId: string): Promise<void>;
}

interface CoreCallbacks {
  onConnected: () => void;
  onDisconnected: () => void;
  onProject: (project: Project) => void;
  onPlaybackState: (playbackState: PlaybackState) => void;
  onProgress: (progress: Progress) => void;
  onProjects: (projects: ProjectInfo[]) => void;
  onWaveform: (waveforms: Map<string, WaveformData>) => void;
}

export const createCore = (callbacks: CoreCallbacks) => {
  let socket: null | WebSocket = null;

  let uploadPromises: {[uploadId: string]: () => void} = {};
  let waitingAcks: string[] = [];
  let pendingRequests: Request[] = [];

  let waveforms = new Map<string, WaveformData>();

  const sendRequest = (request: Request) => {
    if (socket && socket.readyState === WebSocket.OPEN) {
      socket?.send(serialize(request));
    } else {
      pendingRequests.push(request);
    }
  };

  const reconnect = () => {
    if (!socket) {
      socket = new WebSocket(config.WEBSOCKET_ADDRESS);

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
        callbacks.onProject(message.project);
      }

      if (message.playbackState) {
        callbacks.onPlaybackState(message.playbackState);
      }

      if (message.progress) {
        callbacks.onProgress(message.progress);
      }

      if (message.projects) {
        callbacks.onProjects(message.projects);
      }

      if (message.waveform) {
        console.info(
          `Received waveform data for sample ${message.waveform.sampleId}`
        );
        waveforms.set(message.waveform.sampleId, message.waveform.waveformData);
        callbacks.onWaveform(waveforms);
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
    callbacks.onConnected();
    sendRequest(getAllRequest());

    pendingRequests.forEach(sendRequest);
    pendingRequests = [];

    console.log('Connected to core');
  };

  const onClose = () => {
    callbacks.onDisconnected();
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

  reconnect();

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
    reconnect,
  };
};

export type Core = ReturnType<typeof createCore>;
