import {css} from 'styled-components';

export const horizontalGap = (gap: string) => {
  return css`
    & > * {
      margin-right: ${gap};
    }

    & > *:last-child {
      margin-right: 0;
    }
  `;
};

export const verticalGap = (gap: string) => {
  return css`
    & > * {
      margin-bottom: ${gap};
    }

    & > *:last-child {
      margin-bottom: 0;
    }
  `;
};
