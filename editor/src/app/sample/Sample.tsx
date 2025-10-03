import {Button, Chip, Stack} from '@mui/joy';
import {useSampleWithId, useSampleState} from '../../model-hooks/sample-hooks';
import {Delete, FileUpload, Download, Sync, Error} from '@mui/icons-material';
import {useEffect, useRef, useState} from 'react';
import {ID} from '../../api/helpers';
import {useSong} from '../../model-hooks/song-hooks';
import {useDispatcher} from '../../dispatcher/dispatcher';
import {addSampleAction, updateSongAction} from '../../dispatcher/action';
import {spacing} from '../../theme/tokens';

interface Props {
  sampleId: ID;
  songId: ID;
}

export const Sample = ({sampleId, songId}: Props) => {
  const sample = useSampleWithId(sampleId);
  const sampleState = useSampleState(sampleId);
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

  const DownloadStatusChip = () => {
    if (sampleState === 'loading') {
      return (
        <Chip
          variant="soft"
          color="primary"
          startDecorator={<Download />}
          sx={{gap: spacing.xs}}
        >
          Downloading...
        </Chip>
      );
    }

    if (sampleState === 'converting') {
      return (
        <Chip
          variant="soft"
          color="primary"
          startDecorator={<Sync />}
          sx={{gap: spacing.xs}}
        >
          Converting...
        </Chip>
      );
    }

    if (sampleState === 'error') {
      return (
        <Chip
          variant="soft"
          color="danger"
          startDecorator={<Error />}
          sx={{gap: spacing.xs}}
        >
          Download Error
        </Chip>
      );
    }

    return null;
  };

  const getSampleButton = () => {
    if (sample) {
      return <RemoveButton />;
    }

    return <UploadButton />;
  };

  const hasDownloadStatus =
    sampleState === 'loading' ||
    sampleState === 'converting' ||
    sampleState === 'error';

  return (
    <>
      <InvisibleFileInput />
      <Stack direction="row" spacing={spacing.sm} alignItems="center">
        {getSampleButton()}
        {hasDownloadStatus && <DownloadStatusChip />}
      </Stack>
    </>
  );
};
