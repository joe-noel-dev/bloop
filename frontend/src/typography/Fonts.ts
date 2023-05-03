import {createGlobalStyle} from 'styled-components';

import ChivoRegularWoff2 from './fonts/Chivo-Regular.woff2';
import ChivoRegularWoff from './fonts/Chivo-Regular.woff';

export default createGlobalStyle`
  @font-face {
    font-family: 'Chivo';
    src: url(${ChivoRegularWoff2}) format('woff2'), 
      url(${ChivoRegularWoff}) format('woff');
  }
`;
