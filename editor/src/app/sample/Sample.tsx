import {Button} from '@mui/joy';
import {useSampleWithId} from '../../model-hooks/sample-hooks';
import {Delete, FileUpload} from '@mui/icons-material';
import {useEffect, useRef, useState} from 'react';
import {ID} from '../../api/helpers';

interface Props {
  sampleId: ID;
  songId: ID;
}

export const Sample = ({sampleId, songId}: Props) => {
  const sample = useSampleWithId(sampleId);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    if (sample) {
      setUploading(false);
    }
  }, [sample]);

  const remove = () => {
    // FIXME: remove sample
  };

  const onFileSelected = async () => {
    if (fileInputRef.current?.files?.length) {
      const file = fileInputRef.current.files[0];
      setUploading(true);
      // FIXME: add file to song
    }
  };

  const RemoveButton = () => (
    <Button
      color="danger"
      startDecorator={<Delete />}
      onClick={remove}
      variant="soft"
    >
      Remove Sample
    </Button>
  );

  const InvisibleFileInput = () => (
    <input
      type="file"
      accept="audio/wav"
      onChange={onFileSelected}
      ref={fileInputRef}
      style={{display: 'none'}}
    />
  );

  const UploadButton = () => (
    <Button
      loading={uploading}
      color="primary"
      startDecorator={<FileUpload />}
      onClick={() => fileInputRef.current?.click()}
    >
      Upload Sample
    </Button>
  );

  return (
    <>
      {sample && <RemoveButton />}
      {!sample && <InvisibleFileInput />}
      {!sample && <UploadButton />}
    </>
  );
};

// const addSampleToSong = async (file: File, songId: ID, core: Core) => {
//   const uploadId = randomId();

//   const beginRequest = beginUploadRequest(
//     uploadId,
//     file.name,
//     AudioFileFormat.WAV
//   );

//   core.sendRequest(beginRequest);
//   await core.waitForUploadAck(uploadId);

//   const reader = new FileReader();

//   reader.onload = async () => {
//     const result = reader.result as ArrayBuffer;

//     const chunkSize = 1024 * 1024;
//     let position = 0;
//     while (position < result.byteLength) {
//       const chunk = result.slice(position, position + chunkSize);
//       const chunkRequest = uploadRequest(uploadId, chunk);
//       core?.sendRequest(chunkRequest);
//       await core?.waitForUploadAck(uploadId);
//       position += chunkSize;
//     }

//     const completeRequest = completeUploadRequest(uploadId);
//     core?.sendRequest(completeRequest);
//     await core?.waitForUploadAck(uploadId);

//     const addRequest = addSampleRequest(songId, uploadId);
//     core?.sendRequest(addRequest);
//   };

//   reader.readAsArrayBuffer(file);
// };
