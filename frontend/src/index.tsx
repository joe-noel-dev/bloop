import React, {StrictMode} from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import {ThemeProvider} from 'styled-components';
import {appTheme} from './features/theme';
import GlobalFonts from './typography/Fonts';
import ReactModal from 'react-modal';

ReactDOM.render(
  <StrictMode>
    <GlobalFonts />
    <ThemeProvider theme={appTheme}>
      <App />
    </ThemeProvider>
  </StrictMode>,
  document.getElementById('root')
);

ReactModal.setAppElement('#root');
