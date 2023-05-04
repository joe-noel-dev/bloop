import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import styles from './NoConnectionOverlay.module.css';

export const NoConnectionOverlay = () => {
  return (
    <div className={styles['container']}>
      <h1 className={styles['message']}>Waiting for connection...</h1>
      <IndeterminateSpinner />
    </div>
  );
};
