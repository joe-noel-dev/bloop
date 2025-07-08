import {Button} from '@mui/joy';
import {useSampleWithId} from '../../model-hooks/sample-hooks';
import {Delete, FileUpload} from '@mui/icons-material';
import {useEffect, useRef, useState} from 'react';
import {ID} from '../../api/helpers';
import {useSong} from '../../model-hooks/song-hooks';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {addSampleAction, updateSongAction} from '../../dispatcher/action';

interface Props {
  sampleId: ID;
  songId: ID;
}

export const Sample = ({sampleId, songId}: Props) => {
  const sample = useSampleWithId(sampleId);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploading, setUploading] = useState(false);
  const song = useSong(songId);
  const dispatch = useDispatcher();

  useEffect(() => {
    if (sample) {
      setUploading(false);
    }
  }, [sample]);

  if (!song) {
    return <></>;
  }

  const remove = () =>
    dispatch(
      updateSongAction({
        ...song,
        sample: undefined,
      })
    );

  const onFileSelected = () => {
    if (fileInputRef.current?.files?.length) {
      const file = fileInputRef.current.files[0];
      setUploading(true);
      dispatch(addSampleAction(songId, file));
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
