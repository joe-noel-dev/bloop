import {Upload} from '@mui/icons-material';
import {Button} from '@mui/joy';
import {useRef} from 'react';
import pako from 'pako';
import {ID} from '../../api/helpers';
import {Dispatcher, useDispatcher} from '../../dispatcher/dispatcher';
import {addSectionAction} from '../../dispatcher/action';

interface Props {
  songId: ID;
}

export const AbletonUpload = ({songId}: Props) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const dispatch = useDispatcher();

  const onFileSelected = async () => {
    if (!fileInputRef.current?.files?.length) {
      return;
    }

    const file = fileInputRef.current.files[0];
    await uploadFromAls(file, songId, dispatch);
  };

  const InvisibleFileInput = () => (
    <input
      type="file"
      accept=".als"
      onChange={onFileSelected}
      ref={fileInputRef}
      style={{display: 'none'}}
    />
  );

  return (
    <>
      <InvisibleFileInput />
      <Button
        color="primary"
        size="sm"
        variant="soft"
        aria-label="Upload Ableton Project"
        startDecorator={<Upload />}
        onClick={() => fileInputRef.current?.click()}
      >
        Upload Ableton Project
      </Button>
    </>
  );
};

const uploadFromAls = async (
  alsProject: File,
  songId: ID,
  dispatch: Dispatcher
) => {
  const xml = await unzip(alsProject);
  const document = toXmlDocument(xml);
  const locators = getLocators(document);
  console.log('Found locators in Ableton Project:', locators);

  locators.forEach((locator) =>
    dispatch(
      addSectionAction(songId, {
        name: locator.name,
        start: locator.start,
      })
    )
  );
};

const unzip = async (project: File): Promise<string> => {
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = () => {
      const compressed = new Uint8Array(reader.result as ArrayBuffer);
      const decompressed = pako.inflate(compressed, {to: 'string'});
      resolve(decompressed);
    };

    reader.onerror = (error) =>
      reject(`Failed to read file: ${project.name}: ${error}`);

    reader.readAsArrayBuffer(project);
  });
};

const toXmlDocument = (xml: string) => {
  const parser = new DOMParser();
  return parser.parseFromString(xml, 'application/xml');
};

interface Locator {
  name: string;
  start: number;
}

const getLocators = (document: Document) => {
  const locators: Locator[] = [];

  const markers = document.querySelectorAll('Locators > Locators > Locator');

  markers.forEach((marker) => {
    const nameNode = marker.querySelector('Name');
    const timeNode = marker.querySelector('Time');

    if (!nameNode || !timeNode) {
      return;
    }

    const name = nameNode.getAttribute('Value') ?? '';
    const start = parseFloat(timeNode.getAttribute('Value') ?? '0');

    locators.push({
      name,
      start,
    });
  });

  return locators;
};
