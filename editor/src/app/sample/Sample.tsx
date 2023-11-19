import {Sheet, Typography} from '@mui/joy';
import {useSampleWithId} from '../../model-hooks/sample-hooks';

interface Props {
  sampleId: string;
}

export const Sample = ({sampleId}: Props) => {
  const sample = useSampleWithId(sampleId);

  if (!sample) {
    return <></>;
  }

  return (
    <Sheet variant="outlined">
      <Typography>{sample.name}</Typography>
    </Sheet>
  );
};
