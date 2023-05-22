import {SecondaryButton} from '../../components/Button';
import {useCore} from '../core/use-core';
import styles from './NoConnectionOverlay.module.css';

export const NoConnectionOverlay = () => {
  const core = useCore();

  return (
    <div className={styles['container']}>
      <h1 className={styles['message']}>Not connected</h1>

      <SecondaryButton
        className={styles['connect-button']}
        onClick={() => core.reconnect()}
      >
        Connect Now
      </SecondaryButton>
    </div>
  );
};
