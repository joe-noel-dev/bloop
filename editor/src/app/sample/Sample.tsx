import {Box, Button} from '@mui/joy';
import {useSampleWithId} from '../../model-hooks/sample-hooks';
import {Delete, FileUpload} from '@mui/icons-material';
import {
  addSampleRequest,
  beginUploadRequest,
  completeUploadRequest,
  removeSampleRequest,
  uploadRequest,
} from '../../api/request';
import {useCore} from '../../core/use-core';
import {v4 as uuidv4} from 'uuid';
import {useEffect, useRef, useState} from 'react';
import {Core} from '../../core/Core';

interface Props {
  sampleId: string;
  songId: string;
}

export const Sample = ({sampleId, songId}: Props) => {
  const sample = useSampleWithId(sampleId);
  const core = useCore();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploading, setUploading] = useState(false);

  if (!core) {
    return <></>;
  }

  useEffect(() => {
    if (sample) {
      setUploading(false);
    }
  }, [sample]);

  const remove = () => {
    const request = removeSampleRequest(songId);
    core.sendRequest(request);
  };

  const onFileSelected = async () => {
    if (fileInputRef.current?.files?.length) {
      const file = fileInputRef.current.files[0];
      setUploading(true);
      await addSampleToSong(file, songId, core);
    }
  };

  return (
    <Box>
      {sample && (
        <Button color="danger" startDecorator={<Delete />} onClick={remove}>
          Remove Sample
        </Button>
      )}

      {!sample && (
        <input
          type="file"
          accept="audio/wav"
          onChange={onFileSelected}
          ref={fileInputRef}
          style={{display: 'none'}}
        />
      )}

      {!sample && (
        <Button
          loading={uploading}
          color="primary"
          startDecorator={<FileUpload />}
          onClick={() => fileInputRef.current?.click()}
        >
          Upload Sample
        </Button>
      )}
    </Box>
  );
};

const addSampleToSong = async (file: File, songId: string, core: Core) => {
  const uploadId = uuidv4();

  const beginRequest = beginUploadRequest(uploadId, file.name, 'wav');
  core.sendRequest(beginRequest);
  await core.waitForUploadAck(uploadId);

  const reader = new FileReader();

  reader.onload = async () => {
    const result = reader.result as ArrayBuffer;

    const chunkSize = 1024 * 1024;
    let position = 0;
    while (position < result.byteLength) {
      const chunk = result.slice(position, position + chunkSize);
      const chunkRequest = uploadRequest(uploadId, chunk);
      core?.sendRequest(chunkRequest);
      await core?.waitForUploadAck(uploadId);
      position += chunkSize;
    }

    const completeRequest = completeUploadRequest(uploadId);
    core?.sendRequest(completeRequest);
    await core?.waitForUploadAck(uploadId);

    const addRequest = addSampleRequest(songId, uploadId);
    core?.sendRequest(addRequest);
  };

  reader.readAsArrayBuffer(file);
};
