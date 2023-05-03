import {DefaultTheme, keyframes} from 'styled-components';

export const appTheme: DefaultTheme = {
  borderRadius: '2px',

  transition: 'all 0.2s ease-in-out',

  units: (multiplier: number) => `${multiplier * 8}px`,

  dropShadow:
    '0px 1px 3px rgba(0, 0, 0, 0.12), 0px 1px 2px rgba(0, 0, 0, 0.24)',

  dropShadowLevel2:
    '0px 5px 10px rgba(0, 0, 0, 0.2), 0px 6px 6px rgba(0, 0, 0, 0.24)',

  dropShadowLevel3:
    '0px 15px 30px rgba(0, 0, 0, 0.3), 0px 12px 12px rgba(0, 0, 0, 0.24)',

  colours: {
    background: 'white',

    primary: '#ffab91',
    primaryLight: '#ffddc1',
    primaryDark: '#c97b63',

    secondary: '#1a237e',
    secondaryLight: '#534bae',
    secondaryDark: '#000051',

    cardBackground: 'white',
    cardLayer: 'rgb(0, 0, 0, 0.1)',
  },

  textColours: {
    background: 'white',

    primary: 'black',
    primaryLight: 'black',
    primaryDark: 'black',

    secondary: 'white',
    secondaryLight: 'white',
    secondaryDark: 'white',

    card: 'black',
  },

  fadeInKeyFrames: keyframes`
      0% {
        opacity: 0;
      }
      100% {
        opacity: 1;
      }
    `,
};
