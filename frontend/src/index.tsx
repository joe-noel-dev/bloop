import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import './index.css';
import App from './App';
import ReactModal from 'react-modal';

const container = document.getElementById('root');
const root = createRoot(container!);

root.render(
  <StrictMode>
    <App />
  </StrictMode>
);

ReactModal.setAppElement('#root');
