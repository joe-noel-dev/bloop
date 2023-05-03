import {css} from 'styled-components';

export const MainTextStyle = css`
  font-family: 'Chivo', 'sans-serif';
  font-weight: 400;
`;

export const XSmallText = css`
  font-size: 10px;
`;

export const SmallText = css`
  font-size: 12px;
`;

export const MediumText = css`
  font-size: 16px;
`;

export const LargeText = css`
  font-size: 24px;
`;

export const XLargeText = css`
  font-size: 36px;
`;

export const XSmallMain = css`
  ${MainTextStyle};
  ${XSmallText};
`;

export const SmallMain = css`
  ${MainTextStyle};
  ${SmallText};
`;

export const MediumMain = css`
  ${MainTextStyle};
  ${MediumText};
`;

export const LargeMain = css`
  ${MainTextStyle};
  ${LargeText};
`;

export const XLargeMain = css`
  ${MainTextStyle};
  ${XLargeText};
`;
