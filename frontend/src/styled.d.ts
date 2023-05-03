import {Keyframes} from 'styled-components';

declare module 'styled-components' {
  interface TextColours {
    background: string;

    primary: string;
    primaryDark: string;
    primaryLight: string;

    secondary: string;
    secondaryDark: string;
    secondaryLight: string;

    card: string;
  }

  interface Colours {
    background: string;

    primary: string;
    primaryDark: string;
    primaryLight: string;

    secondary: string;
    secondaryDark: string;
    secondaryLight: string;

    cardBackground: string;
    cardLayer: string;
  }

  export interface DefaultTheme {
    borderRadius: string;
    transition: string;

    dropShadow: string;
    dropShadowLevel2: string;
    dropShadowLevel3: string;

    fadeInKeyFrames: Keyframes;
    colours: Colours;
    textColours: TextColours;

    units(multiplier: number): string;
  }
}
