export type ColumnName =
  | 'Play'
  | 'Name'
  | 'Start'
  | 'Loop'
  | 'Metronome'
  | 'Edit';

export const columns: ColumnName[] = [
  'Play',
  'Name',
  'Start',
  'Loop',
  'Metronome',
  'Edit',
];

export const columnSize = (name: ColumnName): number => {
  switch (name) {
    case 'Name':
      return 4;
    case 'Edit':
      return 2;
  }

  return 1;
};
