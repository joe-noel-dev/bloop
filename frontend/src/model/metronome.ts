export type Metronome = 'default' | 'countIn' | 'on' | 'off';

export const toDisplayString = (value: Metronome): string => {
  switch (value) {
    case 'default':
      return 'Default';
    case 'on':
      return 'On';
    case 'off':
      return 'Off';
    case 'countIn':
      return 'Count In';
  }
};
