export type ColumnName =
  | 'Name'
  | 'Start'
  | 'Duration'
  | 'Loop'
  | 'Metronome'
  | 'Edit';

export const columns: ColumnName[] = [
  'Name',
  'Start',
  'Duration',
  'Loop',
  'Metronome',
  'Edit',
];

export const columnSize = (name: ColumnName): number => {
  switch (name) {
    case 'Name':
    case 'Metronome':
    case 'Edit':
      return 2;
    case 'Start':
    case 'Duration':
    case 'Loop':
      return 1;
  }
};
