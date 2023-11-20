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
    case 'Play':
      return 2;
    case 'Name':
      return 2;
    case 'Edit':
      return 2;
  }

  return 1;
};
