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
    case 'Name':
    case 'Metronome':
    case 'Edit':
      return 2;
    case 'Start':
    case 'Loop':
      return 1;
  }
};
